// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//! Tests for [`super`], lifted out so the module reads at a human size.
//!
//! Attached with `#[path]` rather than a directory module: the parent is one
//! compilation unit either way, and a flat layout keeps `use super::*`
//! meaning exactly what it did before the split.

use super::*;

/// watchguard reads its source as `ctx["_temp"].length()`, and the member
/// dot of that CALL used to come back as part of the path.
#[test]
fn a_kv_scan_reads_a_subscripted_source_without_its_call_dot() {
    let script = "def kvStart = 0; def kvSplit = 0; def inQuote = false;\nPattern quotePattern = /^\\\"|\\\"$/;\nfor (int i = 0, n = ctx[\"_temp\"].length(); i < n; ++i) {\n  char c = ctx[\"_temp\"].charAt(i);\n  char c2 = i < n - 1 ? ctx[\"_temp\"].charAt(i + 1) : 0;\n\n  if (c == (char)'\"') {\n    if (inQuote && (c2 == 0 || c2 == (char)' ' || c2 == (char)':')) {\n      inQuote = false;\n    } else {\n      inQuote = true;\n    }\n  }\n  if (inQuote) {\n    continue;\n  }\n  if (c == (char)'=') {\n    kvSplit = i;\n  }\n  if (c == (char)'\"' || c == (char)' ' || c2 == 0) {\n    if (i != kvStart) {\n      def endIndex = i == n - 1 ? i + 1 : i;\n      def key = ctx[\"_temp\"].substring(kvStart, kvSplit);\n      def value = quotePattern.matcher(ctx[\"_temp\"].substring(kvSplit + 1, endIndex)).replaceAll(\"\");\n\n      if (key != '') {\n        ctx.watchguard_firebox.log.put(key, value);\n      }\n    }\n\n    kvStart = i + 1;\n    kvSplit = i + 1;\n  }\n}";
    let normalised = normalise(script);
    let found = known_patterns(&normalised);
    let mut event = Event::new(serde_json::json!({
        "_temp": "msg=\"HTTP request\" proxy_act=\"HTTP-Client.Standard.1\" op=\"GET\"",
        "watchguard_firebox": { "log": {} }
    }));
    assert!(
        found
            .iter()
            .any(|pattern| run_known_pattern(&mut event, &normalised, pattern)),
        "the scanner must claim the script it was written for"
    );
    assert_eq!(
        event.get("watchguard_firebox.log.msg"),
        Some(&Value::from("HTTP request"))
    );
    assert_eq!(
        event.get("watchguard_firebox.log.proxy_act"),
        Some(&Value::from("HTTP-Client.Standard.1"))
    );
    assert_eq!(
        event.get("watchguard_firebox.log.op"),
        Some(&Value::from("GET"))
    );
}

/// `GeoJSON` puts longitude FIRST, so the indices come off the script.
#[test]
fn a_geo_point_reads_its_position_indices_from_the_script() {
    let script = "def geom = ctx.geometry;\nString geomType = geom.type;\n\
        if (geomType == \"Point\" && geom.coordinates != null) {\n    \
        ctx.gdacs.geo.location = ['lon': geom.coordinates[0], 'lat': geom.coordinates[1]];\n}\n";
    let pattern =
        parse_geo_point_from_coordinates(&normalise(script)).expect("gdacs writes a point");
    assert_eq!(pattern.coordinates, "geometry.coordinates");
    assert_eq!(pattern.target, "gdacs.geo.location");
    assert_eq!((pattern.lon_index, pattern.lat_index), (0, 1));

    let mut event = Event::new(serde_json::json!({
        "geometry": { "type": "Point", "coordinates": [138.2, -34.9] }
    }));
    assert!(geo_point_from_coordinates(&mut event, &pattern));
    assert_eq!(
        event.get("gdacs.geo.location"),
        Some(&serde_json::json!({ "lon": 138.2, "lat": -34.9 }))
    );

    // A Polygon's position is itself an array, which is the case the
    // vendor's `== "Point"` guard excludes, so nothing is written.
    let mut polygon = Event::new(serde_json::json!({
        "geometry": { "type": "Polygon", "coordinates": [[[1.0, 2.0]]] }
    }));
    assert!(geo_point_from_coordinates(&mut polygon, &pattern));
    assert_eq!(polygon.get("gdacs.geo.location"), None);
}

/// gdacs guards on a member of a local, and guards with more than one
/// clause -- the two things that made every copy in the branch dead.
#[test]
fn a_branch_guards_on_a_local_and_keeps_its_namespace() {
    let script = "def polyGeom = ctx.polygon_geometry;\n\
        String polyType = polyGeom.type;\n\
        if (polyType == \"Polygon\") {\n  \
        if (ctx.polygon_class != null) { ctx.gdacs.class = ctx.polygon_class; }\n  \
        if (ctx.polygon_label != null && ctx.polygon_label != \"\") \
        { ctx.gdacs.polygon_label = ctx.polygon_label; }\n}\n";
    let branches = parse_branch_copies(&normalise(script)).expect("gdacs guards on a type");
    assert_eq!(branches[0].guard, "polygon_geometry.type");
    assert_eq!(
        branches[0].copies,
        [
            ("gdacs.class".to_owned(), "polygon_class".to_owned()),
            ("gdacs.polygon_label".to_owned(), "polygon_label".to_owned()),
        ]
    );

    let mut event = Event::new(serde_json::json!({
        "polygon_geometry": { "type": "Polygon" },
        "polygon_class": "Poly_Affected",
        "polygon_label": "Population affected"
    }));
    assert!(run_branch_copies(&mut event, &branches));
    assert_eq!(
        event.get("gdacs.class"),
        Some(&Value::from("Poly_Affected"))
    );
    assert_eq!(
        event.get("gdacs.polygon_label"),
        Some(&Value::from("Population affected"))
    );
}

/// gdacs folds two flags, and an unrecognised spelling becomes FALSE --
/// the difference from `isTruthy`, which would leave it alone.
#[test]
fn a_flag_folds_to_a_boolean_by_its_spelling() {
    let script = "if (ctx.gdacs?.is_current != null) {\n  \
        def val = ctx.gdacs.is_current.toString().toLowerCase();\n  \
        ctx.gdacs.is_current = (val == \"true\" || val == \"1\");\n}\n\
        if (ctx.gdacs?.is_temporary != null) {\n  \
        def val = ctx.gdacs.is_temporary.toString().toLowerCase();\n  \
        ctx.gdacs.is_temporary = (val == \"true\" || val == \"1\");\n}\n";
    let pattern = parse_coerce_boolean(&normalise(script)).expect("gdacs folds two flags");
    assert_eq!(pattern.truthy, ["true", "1"]);
    assert_eq!(
        pattern.fields,
        [
            ("gdacs.is_current".to_owned(), "gdacs.is_current".to_owned()),
            (
                "gdacs.is_temporary".to_owned(),
                "gdacs.is_temporary".to_owned()
            ),
        ]
    );

    let mut event = Event::new(serde_json::json!({
        "gdacs": { "is_current": "True", "is_temporary": "no" }
    }));
    assert!(coerce_boolean(&mut event, &pattern));
    assert_eq!(event.get("gdacs.is_current"), Some(&Value::Bool(true)));
    assert_eq!(event.get("gdacs.is_temporary"), Some(&Value::Bool(false)));
}

/// gitlab names every measurement `<thing>.values` and lists the reading.
///
/// Verbatim from `pipelines/gitlab/application/default.yml:84`. Leaving it
/// unbound put every measurement one level too deep -- 688 missing fields
/// matched by 688 extra ones, which is the raw-shape signature exactly.
#[test]
fn a_suffixed_key_is_rewritten_without_it_and_a_lone_value_unwrapped() {
    let script = "if (ctx.gitlab?.application != null) {\n  \
        def fieldsToRename = new ArrayList(ctx.gitlab.application.keySet());\n  \
        for (fieldName in fieldsToRename) {\n    \
        if (fieldName.endsWith('values')) {\n      \
        def newField = fieldName.substring(0, fieldName.length() - 7);\n      \
        def value = ctx.gitlab.application[fieldName];\n      \
        if (value.size() > 1) {\n        \
        ctx.gitlab.application[newField] = value;\n      } else {\n        \
        ctx.gitlab.application[newField] = value[0]\n      }\n      \
        ctx.gitlab.application.remove(fieldName);\n    }\n  }\n}";
    let pattern =
        parse_unwrap_suffixed_keys(&normalise(script)).expect("gitlab renames its measurements");
    assert_eq!(pattern.container, "gitlab.application");
    assert_eq!(pattern.suffix, "values");
    assert_eq!(pattern.trim, 7);

    let mut event = Event::new(serde_json::json!({ "gitlab": { "application": {
        "mergeability.check_approved_service.db_count.values": [1],
        "mergeability.check_broken_status_service.duration_s.values": [0.5, 0.75],
        "correlation_id": "01J0PF6DFMXRC0JJK70AG21DJD"
    } } }));
    assert!(unwrap_suffixed_keys(&mut event, &pattern));

    // One reading comes OUT of its list, under the key without the suffix.
    assert_eq!(
        event.get("gitlab.application.mergeability.check_approved_service.db_count"),
        Some(&serde_json::json!(1))
    );
    // Several stay a list.
    assert_eq!(
        event.get("gitlab.application.mergeability.check_broken_status_service.duration_s"),
        Some(&serde_json::json!([0.5, 0.75]))
    );
    // A key without the suffix is untouched, and the old keys are gone.
    assert_eq!(
        event.get_str("gitlab.application.correlation_id"),
        Some("01J0PF6DFMXRC0JJK70AG21DJD")
    );
    let keys: Vec<&String> = event
        .get_object("gitlab.application")
        .map(|m| m.keys().collect())
        .unwrap_or_default();
    assert!(!keys.iter().any(|k| k.ends_with("values")), "{keys:?}");
}

/// Google Workspace ships each application's detail as a parameter list.
///
/// Verbatim from `pipelines/google_workspace/calendar/default.yml:60`. Seven
/// of its streams write the same loop, and the whole per-application block is
/// absent without it. The value keys are tried in the script's own order,
/// because that is what its `else if` ladder does.
#[test]
fn a_parameter_list_fans_out_into_a_map_under_each_name() {
    let script = "ctx.google_workspace = ctx.google_workspace ?: [:]; \
        ctx.google_workspace.calendar = ctx.google_workspace.calendar ?: [:];\n\
        for (def param : ctx.json.events.parameters) {\n  \
        if (param.name == null) {\n    continue;\n  }\n  \
        def lw_case_name = param.name.toLowerCase();\n  \
        if (param.value != null) {\n    \
        ctx.google_workspace.calendar[lw_case_name] = param.value;\n  \
        } else if (param.boolValue != null) {\n    \
        ctx.google_workspace.calendar[lw_case_name] = param.boolValue;\n  \
        } else if (param.multiValue != null) {\n    \
        ctx.google_workspace.calendar[lw_case_name] = param.multiValue;\n  }\n}";
    let pattern = parse_parameters_into_map(&normalise(script)).expect("google_workspace fans out");
    assert_eq!(pattern.source, "json.events.parameters");
    assert_eq!(pattern.target, "google_workspace.calendar");
    assert_eq!(pattern.name_key, "name");
    assert!(pattern.lowercase);
    assert_eq!(pattern.value_keys, ["value", "boolValue", "multiValue"]);

    let mut event = Event::new(serde_json::json!({
        "json": { "events": { "parameters": [
            { "name": "calendar_id", "value": "cal-1" },
            { "name": "API_KIND", "value": "event" },
            { "name": "is_recurring", "boolValue": true },
            { "name": "grantees", "multiValue": ["a@x.com", "b@x.com"] },
            { "name": "nothing_here" },
            { "value": "no name at all" }
        ] } },
        "google_workspace": { "calendar": { "already": "kept" } }
    }));
    assert!(parameters_into_map(&mut event, &pattern));

    // The NAME is case-folded, the value is not.
    assert_eq!(
        event.get_str("google_workspace.calendar.calendar_id"),
        Some("cal-1")
    );
    assert_eq!(
        event.get_str("google_workspace.calendar.api_kind"),
        Some("event")
    );
    assert_eq!(
        event.get("google_workspace.calendar.is_recurring"),
        Some(&serde_json::json!(true))
    );
    assert_eq!(
        event.get("google_workspace.calendar.grantees"),
        Some(&serde_json::json!(["a@x.com", "b@x.com"]))
    );
    // A parameter with no value key, and one with no name, are both skipped.
    assert!(!event.has("google_workspace.calendar.nothing_here"));
    // MERGED into what was already there.
    assert_eq!(
        event.get_str("google_workspace.calendar.already"),
        Some("kept")
    );
}

/// `juniper_srx` swaps a hyphen for an underscore across every key it has.
///
/// Verbatim from `pipelines/juniper_srx/log`. This is NOT the camel-case
/// converter: `snake_case_apply` declines a helper with no
/// `Character.isUpperCase` in it, which is right, and left this bound to
/// nothing until it had an arm of its own.
#[test]
fn every_key_of_a_map_takes_one_character_replacement() {
    let script = "ctx.juniper.srx = ctx?.juniper?.srx.entrySet().stream()\
        .collect(Collectors.toMap(e -> e.getKey().replace('-', '_'), e -> e.getValue()));";
    let pattern = parse_rename_map_keys(&normalise(script)).expect("juniper_srx rewrites keys");
    assert_eq!(pattern.container, "juniper.srx");
    assert_eq!(pattern.from, '-');
    assert_eq!(pattern.to, '_');

    let mut event = Event::new(serde_json::json!({ "juniper": { "srx": {
        "source-address": "10.0.0.1",
        "destination-port": 443,
        "already_fine": "x"
    } } }));
    assert!(rename_map_keys(&mut event, &pattern));
    assert_eq!(
        event.get_str("juniper.srx.source_address"),
        Some("10.0.0.1")
    );
    assert_eq!(
        event.get("juniper.srx.destination_port"),
        Some(&serde_json::json!(443))
    );
    // A key with nothing to replace is carried through untouched.
    assert_eq!(event.get_str("juniper.srx.already_fine"), Some("x"));
    assert!(!event.has("juniper.srx.source-address"));
}

/// oracle folds every key to lower case in the same one-liner form, and the
/// `rename` processors after it all name the lowercase spelling.
///
/// Verbatim from `pipelines/oracle/database_audit/default.yml`.
#[test]
fn every_key_of_a_map_takes_a_chain_of_transforms() {
    let script = "ctx.oracle.database_audit = ctx.oracle.database_audit.entrySet()\
        .stream().collect(Collectors.toMap(entry -> entry.getKey().toLowerCase(), \
        Map.Entry::getValue));";
    let pattern = parse_stream_rewrite_keys(&normalise(script)).expect("oracle folds its keys");

    let mut event = Event::new(serde_json::json!({ "oracle": { "database_audit": {
        "DBID": "2824230686",
        "SESSIONID": "0",
        "USERHOST": "testlab.local"
    } } }));
    assert!(rewrite_keys(&mut event, &pattern));
    assert_eq!(
        event.get_str("oracle.database_audit.dbid"),
        Some("2824230686")
    );
    assert_eq!(event.get_str("oracle.database_audit.sessionid"), Some("0"));
    assert_eq!(
        event.get_str("oracle.database_audit.userhost"),
        Some("testlab.local")
    );
    assert!(!event.has("oracle.database_audit.DBID"));
}

/// tetragon's event is one of seven `process_*` keys, and the pipeline lifts
/// the same members out of whichever arrived so the twenty renames after it can
/// name ONE path.
///
/// Verbatim from `pipelines/tetragon/log/default.yml`. The two members differ in
/// the keys they accept -- `process_loader` carries a process and no parent --
/// which is why the keys are read per member rather than once.
#[test]
fn a_member_is_lifted_from_whichever_key_arrived() {
    let script = "void run(Map map) {\n  for (def k : map?.cilium_tetragon?.log?.keySet()) {\n    \
        /* these tetragon objects have \"process\" */\n    if (k == \"process_exec\" ||\n        \
        k == \"process_exit\" ||\n        k == \"process_loader\") {\n      \
        if (map?._tmp_ == null) {\n        map[\"_tmp_\"] = new HashMap();\n      }\n      \
        map[\"_tmp_\"][\"process\"] = map.cilium_tetragon.log[k].process;\n    }\n\n    \
        /* these tetragon objects have \"parent\" */\n    if (k == \"process_exec\" ||\n        \
        k == \"process_exit\") {\n      if (map?._tmp_ == null) {\n        \
        map[\"_tmp_\"] = new HashMap();\n      }\n      \
        map[\"_tmp_\"][\"parent\"] = map.cilium_tetragon.log[k].parent;\n    }\n  }\n}\n\nrun(ctx);\n";
    let pattern =
        parse_member_from_variant_key(&normalise(script)).expect("the lift is recognised");

    let mut event = Event::new(serde_json::json!({ "cilium_tetragon": { "log": {
        "process_exec": {
            "process": { "pid": 224_395, "binary": "/usr/local/bin/x" },
            "parent": { "pid": 223_965 }
        }
    } } }));
    assert!(member_from_variant_key(&mut event, &pattern));
    assert_eq!(
        event.get("_tmp_.process.pid"),
        Some(&serde_json::json!(224_395))
    );
    assert_eq!(
        event.get("_tmp_.parent.pid"),
        Some(&serde_json::json!(223_965))
    );

    // A different variant key, and the member that one does not carry.
    let mut loader = Event::new(serde_json::json!({ "cilium_tetragon": { "log": {
        "process_loader": { "process": { "pid": 7 } }
    } } }));
    assert!(member_from_variant_key(&mut loader, &pattern));
    assert_eq!(loader.get("_tmp_.process.pid"), Some(&serde_json::json!(7)));
    assert!(!loader.has("_tmp_.parent"));

    // A key outside both lists is left alone.
    let mut other = Event::new(serde_json::json!({ "cilium_tetragon": { "log": {
        "test_sensor": { "process": { "pid": 9 } }
    } } }));
    assert!(member_from_variant_key(&mut other, &pattern));
    assert!(!other.has("_tmp_"));
}

/// ti_flashpoint rewrites every key at every depth and MOVES the result, so a
/// one-level reader would leave the nested maps spelt the vendor's way and the
/// payload sitting under `json.*` as well.
///
/// Verbatim from `pipelines/ti_flashpoint/alert/default.yml`.
#[test]
fn a_whole_subtree_is_rewritten_and_moved() {
    let script = "String normalize(String str) {\n  return str.replace('-', '_');\n}\n\
        def normalizeFields(def obj) {\n  if (obj instanceof Map) {\n    \
        def newObj = new HashMap();\n    for (entry in obj.entrySet()) {\n      \
        String newKey = normalize(entry.getKey());\n      \
        newObj.put(newKey, normalizeFields(entry.getValue()));\n    }\n    \
        return newObj;\n  } else if (obj instanceof List) {\n    \
        def newList = new ArrayList();\n    for (item in obj) {\n      \
        newList.add(normalizeFields(item));\n    }\n    return newList;\n  }\n  \
        return obj;\n}\n\nif (ctx.json != null) {\n  \
        ctx.ti_flashpoint = ctx.ti_flashpoint ?: [:];\n  \
        ctx.ti_flashpoint.alert = normalizeFields(ctx.json);\n  \
        ctx.remove('json');\n}";
    let pattern = parse_recursive_rewrite_keys(&normalise(script)).expect("the move is recognised");

    let mut event = Event::new(serde_json::json!({ "json": {
        "id": "a-1",
        "reason": { "some-key": "v", "list": [{ "nested-key": 1 }] }
    } }));
    assert!(rewrite_keys(&mut event, &pattern));

    assert_eq!(event.get_str("ti_flashpoint.alert.id"), Some("a-1"));
    // Every depth, lists included.
    assert_eq!(
        event.get_str("ti_flashpoint.alert.reason.some_key"),
        Some("v")
    );
    assert_eq!(
        event
            .get("ti_flashpoint.alert.reason.list")
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("nested_key")),
        Some(&serde_json::json!(1))
    );
    // The source is taken away, or the whole payload ships twice.
    assert!(!event.has("json"));
}

/// The division between the two readers, and the one script neither may claim.
///
/// `juniper_srx`'s system stream chains two replacements and a case fold on the
/// key and trims every value too. `RewriteKeys` does not model the value step, so
/// claiming it would write the keys right and the values wrong -- which reads
/// as a source needing polish rather than one needing a different pattern.
#[test]
fn a_fold_it_cannot_finish_is_declined_rather_than_half_applied() {
    let single = "ctx.juniper.srx = ctx?.juniper?.srx.entrySet().stream()\
        .collect(Collectors.toMap(e -> e.getKey().replace('-', '_'), e -> e.getValue()));";
    assert!(parse_stream_rewrite_keys(&normalise(single)).is_none());
    assert!(parse_rename_map_keys(&normalise(single)).is_some());

    let trims_the_value = "ctx.juniper.srx.system = ctx.juniper.srx.system.entrySet().stream()\
        .collect(Collectors.toMap(e -> e.getKey().replace(' ', '_').replace('-', '_')\
        .toLowerCase(), e -> e.getValue().trim()));";
    assert!(parse_stream_rewrite_keys(&normalise(trims_the_value)).is_none());
}

/// A `keysToSnakeCase` script converts the container it NAMES, nothing else.
///
/// Three vendors, three spellings, all verbatim: tanium names a nested path,
/// cyberarkpas spells the helper `keys_to_snake_case_recursive`, and
/// cloudflare writes the target as a quoted subscript. Each is given here with
/// its newlines ESCAPED, which is how a stored script actually arrives -- as
/// ONE line, so anything reading it line by line lands inside the helper body.
#[test]
fn a_snake_case_script_converts_only_the_container_it_names() {
    let tanium = r"Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    }\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\n\nif(ctx.tanium?.threat_response?.state != null) {\n  ctx.tanium.threat_response.state = keysToSnakeCase(ctx.tanium.threat_response.state);\n}\n";
    assert_eq!(
        snake_case_target(tanium).as_deref(),
        Some("tanium.threat_response.state")
    );

    let cyberark = r"def keys_to_snake_case_recursive(Map object) {\n  return object.entrySet();\n}\nctx.cyberarkpas.monitor = keys_to_snake_case_recursive(ctx.cyberarkpas.monitor);\n";
    assert_eq!(
        snake_case_target(cyberark).as_deref(),
        Some("cyberarkpas.monitor")
    );

    let cloudflare = r"Map keysToSnakeCase(Map m) {\n  return m;\n}\nctx.cloudflare_logpush['workers_trace'] = keysToSnakeCase(ctx.cloudflare_logpush.workers_trace);\n";
    assert_eq!(
        snake_case_target(cloudflare).as_deref(),
        Some("cloudflare_logpush.workers_trace")
    );

    // The whole point: a payload sitting elsewhere is UNTOUCHED. tanium's keys
    // are `Computer IP` and `Event Id`, and converting the whole document
    // turned them into `computer _i_p`, leaving every later read empty.
    let mut event = Event::new(serde_json::json!({
        "json": { "Computer IP": "81.2.69.192", "Computer Name": "worker-2" },
        "tanium": { "threat_response": { "state": { "connectionId": "c" } } }
    }));
    assert!(try_known_painless(&mut event, tanium));
    assert_eq!(event.get_str("json.Computer IP"), Some("81.2.69.192"));
    assert_eq!(event.get_str("json.Computer Name"), Some("worker-2"));
    assert_eq!(
        event.get_str("tanium.threat_response.state.connection_id"),
        Some("c")
    );

    // A script naming nothing this can resolve writes NOTHING.
    assert_eq!(snake_case_target("keysToSnakeCase(m);"), None);
}

/// A key that holds a dot moves by literal key, in both its spellings.
///
/// Verbatim from `pipelines/cybereason/suspicions_process` and
/// `pipelines/f5_bigip/log/pipeline_bigipsystem.yml:699`. A `rename` cannot do
/// this: `imageFile.md5String` is one key, and a dotted path reads two fields.
#[test]
fn a_dotted_key_moves_out_of_its_map_by_that_key() {
    let direct = "def obj = ctx.json.simpleValues.remove(\"imageFile.md5String\"); \
        ctx.cybereason.suspicions_process.simple_values.image_file_md5_string = obj;";
    let pattern = parse_move_map_entry(&normalise(direct)).expect("cybereason moves one key");
    assert_eq!(pattern.container, "json.simpleValues");
    assert_eq!(pattern.key, "imageFile.md5String");
    assert_eq!(
        pattern.target,
        "cybereason.suspicions_process.simple_values.image_file_md5_string"
    );

    let mut event = Event::new(serde_json::json!({ "json": { "simpleValues": {
        "imageFile.md5String": "d41d8cd98f00b204e9800998ecf8427e",
        "imageFile.productName": "kept"
    } } }));
    assert!(move_map_entry(&mut event, &pattern));
    assert_eq!(
        event.get_str("cybereason.suspicions_process.simple_values.image_file_md5_string"),
        Some("d41d8cd98f00b204e9800998ecf8427e")
    );
    // MOVED: the original key goes, and its neighbour is untouched.
    assert_eq!(
        event.get("json.simpleValues"),
        Some(&serde_json::json!({ "imageFile.productName": "kept" }))
    );

    // The `put` spelling lands in the same place.
    let via_put = "def client_side_traffic = new HashMap(); \
        def obj = ctx.json.system.tmmTraffic.remove('clientSideTraffic.bitsIn'); \
        client_side_traffic.put('bits_in', obj); \
        if (ctx.f5_bigip?.log?.tmm_traffic == null) {\n  \
        ctx.f5_bigip.log.tmm_traffic = new HashMap();\n  \
        ctx.f5_bigip.log.tmm_traffic.put('client_side_traffic', client_side_traffic);\n}\nelse{\n  \
        ctx.f5_bigip.log.tmm_traffic.client_side_traffic.put('bits_in', obj);\n}";
    let f5 = parse_move_map_entry(&normalise(via_put)).expect("f5_bigip moves one key");
    assert_eq!(f5.container, "json.system.tmmTraffic");
    assert_eq!(f5.key, "clientSideTraffic.bitsIn");
    assert_eq!(
        f5.target,
        "f5_bigip.log.tmm_traffic.client_side_traffic.bits_in"
    );

    let mut traffic = Event::new(serde_json::json!({ "json": { "system": { "tmmTraffic": {
        "clientSideTraffic.bitsIn": 1234
    } } } }));
    assert!(move_map_entry(&mut traffic, &f5));
    assert_eq!(
        traffic.get("f5_bigip.log.tmm_traffic.client_side_traffic.bits_in"),
        Some(&serde_json::json!(1234))
    );

    // A key the document lacks is not handled, and nothing is created.
    let mut absent = Event::new(serde_json::json!({ "json": { "simpleValues": {} } }));
    assert!(!move_map_entry(&mut absent, &pattern));
    assert!(!absent.has("cybereason.suspicions_process.simple_values.image_file_md5_string"));
}

/// `eset_protect` ships a whole mail as one `mailto:` URI.
///
/// Verbatim from `pipelines/eset_protect/event/default.yml:569`. Only the
/// bracketed addresses are taken -- the display name around them is not one.
#[test]
fn an_email_block_comes_out_of_a_mailto_uri() {
    let script = "String uri = ctx.eset_protect.event.object_uri;\n\
        java.util.regex.Matcher fromMatch = /(?:\\?|&)from=([^&]+)/.matcher(uri);\n\
        java.util.regex.Matcher subjectMatch = /(?:\\?|&)subject=([^&]+)/.matcher(uri);\n\
        java.util.regex.Matcher attachmentMatch = /(?:\\?|&)attachment=([^&]+)/.matcher(uri);\n\
        ctx.email['from'] = from;\nctx.email['subject'] = subject;\n\
        ctx.email['attachments'] = attachmentList;";
    let pattern = parse_mailto_uri_fields(&normalise(script)).expect("eset_protect reads a mailto");
    assert_eq!(pattern.source, "eset_protect.event.object_uri");
    assert_eq!(pattern.target, "email");

    let mut event = Event::new(
        serde_json::json!({ "eset_protect": { "event": { "object_uri":
        "mailto:x@y.com?from=Alice <alice@example.com>, Bob <bob@example.org>\
         &subject=Quarterly&attachment=report.pdf" } } }),
    );
    assert!(mailto_uri_fields(&mut event, &pattern));
    assert_eq!(
        event.get("email.from.address"),
        Some(&serde_json::json!(["alice@example.com", "bob@example.org"]))
    );
    assert_eq!(event.get_str("email.subject"), Some("Quarterly"));
    assert_eq!(
        event.get("email.attachments"),
        Some(&serde_json::json!([{ "file": { "name": "report.pdf" } }]))
    );

    // A URI carrying none of the three writes nothing at all.
    let mut bare = Event::new(serde_json::json!({ "eset_protect": { "event": {
        "object_uri": "mailto:x@y.com"
    } } }));
    assert!(!mailto_uri_fields(&mut bare, &pattern));
    assert!(!bare.has("email.subject"));

    // A `from` with no angle brackets yields no address.
    let mut plain = Event::new(serde_json::json!({ "eset_protect": { "event": {
        "object_uri": "mailto:x@y.com?from=alice@example.com"
    } } }));
    assert!(!mailto_uri_fields(&mut plain, &pattern));
    assert!(!plain.has("email.from.address"));
}

/// The two cuts a dissect leaves behind, both from envoyproxy.
///
/// Verbatim from `pipelines/envoyproxy/log/plaintext.yml:30` and
/// `http.yml:7`. Both became reachable only once the prefix arm made the
/// dissect succeed. Note the SPACE the vendor writes before `indexOf`'s paren.
#[test]
fn a_field_splits_at_its_delimiter_into_named_targets() {
    let with_sentinel = "if (ctx.dest == \"-\") {\n  ctx.remove('dest');\n} else {\n  \
        ctx['destination'] = new HashMap();\n  def p = ctx.dest.indexOf (':');\n  \
        def l = ctx.dest.length();\n  ctx.destination.address = ctx.dest.substring(0, p);\n  \
        ctx.destination.port = ctx.dest.substring(p+1, l);\n}\nctx.remove('dest');";
    let pattern =
        parse_split_at_delimiter(&normalise(with_sentinel)).expect("envoyproxy cuts dest");
    assert_eq!(pattern.source, "dest");
    assert_eq!(pattern.delimiter, ":");
    assert_eq!(pattern.head.as_deref(), Some("destination.address"));
    assert_eq!(pattern.tail.as_deref(), Some("destination.port"));
    assert_eq!(pattern.sentinel.as_deref(), Some("-"));
    assert!(pattern.remove_source);

    let mut event = Event::new(serde_json::json!({ "dest": "172.27.0.2:80" }));
    assert!(split_at_delimiter(&mut event, &pattern));
    assert_eq!(event.get_str("destination.address"), Some("172.27.0.2"));
    assert_eq!(event.get_str("destination.port"), Some("80"));
    // READ, then removed -- leaving it behind emits a field Elastic does not.
    assert!(!event.has("dest"));

    // The sentinel drops the source and writes nothing.
    let mut dash = Event::new(serde_json::json!({ "dest": "-" }));
    assert!(split_at_delimiter(&mut dash, &pattern));
    assert!(!dash.has("dest"));
    assert!(!dash.has("destination.address"));

    // No delimiter is NOT handled: `indexOf` gives -1 and the vendor's
    // `substring` throws on it.
    let mut bare = Event::new(serde_json::json!({ "dest": "172.27.0.2" }));
    assert!(!split_at_delimiter(&mut bare, &pattern));
    assert!(bare.has("dest"));

    // The tail-only form, which keeps its source.
    let tail_only = "ctx['http'] = new HashMap(); def p = ctx.proto.indexOf ('/'); \
        def l = ctx.proto.length(); ctx.http.version = ctx.proto.substring(p+1, l);";
    let proto = parse_split_at_delimiter(&normalise(tail_only)).expect("envoyproxy cuts proto");
    assert_eq!(proto.source, "proto");
    assert_eq!(proto.delimiter, "/");
    assert_eq!(proto.head, None);
    assert_eq!(proto.tail.as_deref(), Some("http.version"));
    assert!(!proto.remove_source);

    let mut http = Event::new(serde_json::json!({ "proto": "HTTP/1.1" }));
    assert!(split_at_delimiter(&mut http, &proto));
    assert_eq!(http.get_str("http.version"), Some("1.1"));
    assert_eq!(http.get_str("proto"), Some("HTTP/1.1"));

    // A receiver that is a local, not a ctx path, is refused: sentinel_one
    // binds `def path = ...` and cuts it four ways, and reading back to the
    // last `ctx.` captures half a statement that still parses.
    let local_receiver = "def path = ctx.json.tgt.file.path;\n\
        int idx = path.lastIndexOf('/');\nif (idx > -1) {\n  \
        ctx.file.name = path.substring(idx+1);\n  \
        ctx.file.directory = path.substring(0, idx);\n}\n\
        if (path.indexOf(':') == 1) {\n  \
        ctx.file.drive_letter = path.substring(0, 1).toUpperCase();\n}";
    assert_eq!(parse_split_at_delimiter(&normalise(local_receiver)), None);
}

/// envoyproxy normalises its access log before one dissect reads both forms.
///
/// Verbatim from `pipelines/envoyproxy/log/plaintext.yml:8`. Everything
/// downstream reads the prefixed copy, so leaving it unbound cost the whole
/// source -- 7 events and 137 fields.
#[test]
fn a_message_is_normalised_to_carry_a_known_prefix() {
    let script = "if (ctx.message.charAt(0) == (char)(\"[\")) {\n  \
        ctx.temp_message = \"ACCESS \" + ctx.message;\n\
        } else if (ctx.message.substring(0, 7) == \"ACCESS \") {\n  \
        ctx.temp_message = ctx.message;\n\
        } else {\n  \
        throw new Exception(\"Not a valid envoyproxy access log\");\n}";
    let pattern = parse_ensure_prefix(&normalise(script)).expect("envoyproxy normalises");
    assert_eq!(pattern.source, "message");
    assert_eq!(pattern.target, "temp_message");
    assert_eq!(pattern.marker, '[');
    assert_eq!(pattern.prefix, "ACCESS ");

    // The bare form GAINS the prefix.
    let mut bare = Event::new(serde_json::json!({ "message": "[2025-01-01] \"GET / HTTP/1.1\"" }));
    assert!(ensure_prefix(&mut bare, &pattern));
    assert_eq!(
        bare.get_str("temp_message"),
        Some("ACCESS [2025-01-01] \"GET / HTTP/1.1\"")
    );

    // The prefixed form passes through unchanged, not doubled.
    let mut already = Event::new(serde_json::json!({ "message": "ACCESS [2025-01-01] x" }));
    assert!(ensure_prefix(&mut already, &pattern));
    assert_eq!(
        already.get_str("temp_message"),
        Some("ACCESS [2025-01-01] x")
    );

    // Neither form is NOT handled -- the vendor throws, and claiming it would
    // count a script this arm did not apply.
    let mut other = Event::new(serde_json::json!({ "message": "something else" }));
    assert!(!ensure_prefix(&mut other, &pattern));
    assert!(!other.has("temp_message"));

    // No message at all is not handled either.
    let mut absent = Event::new(serde_json::json!({ "other": 1 }));
    assert!(!ensure_prefix(&mut absent, &pattern));
}

/// sysdig's tidy-up after a dot expansion.
///
/// Verbatim from `pipelines/sysdig/event/default.yml:217`. Expanding
/// `proc.pid.ts` builds a nested `proc.pid`, the rename takes the leaf away,
/// and `proc.pid` is left holding `{}` -- a container Elasticsearch does not
/// emit. Only an EMPTY map goes: a populated one is real data.
#[test]
fn a_named_key_holding_an_empty_map_is_dropped() {
    let script = "if (ctx.sysdig.event.content.fields.proc instanceof Map) {\n  \
        def proc = ctx.sysdig.event.content.fields.proc;\n  \
        if (proc.containsKey('pid') && proc.pid instanceof Map && proc.pid.size() == 0) {\n    \
        proc.remove('pid');\n  }\n  \
        if (proc.containsKey('ppid') && proc.ppid instanceof Map && proc.ppid.size() == 0) {\n    \
        proc.remove('ppid');\n  }\n}\n";
    let pattern =
        parse_remove_empty_child_maps(&normalise(script)).expect("sysdig drops two empty maps");
    assert_eq!(pattern.container, "sysdig.event.content.fields.proc");
    assert_eq!(pattern.keys, ["pid", "ppid"]);

    let mut event = Event::new(serde_json::json!({ "sysdig": { "event": { "content": {
        "fields": { "proc": {
            "pid": {},
            "ppid": { "ts": "kept -- not empty" },
            "name": "sh"
        } }
    } } } }));
    assert!(remove_empty_child_maps(&mut event, &pattern));

    assert!(!event.has("sysdig.event.content.fields.proc.pid"));
    // A populated map under a named key stays, and so does everything unnamed.
    assert_eq!(
        event.get("sysdig.event.content.fields.proc.ppid.ts"),
        Some(&Value::from("kept -- not empty"))
    );
    assert_eq!(
        event.get("sysdig.event.content.fields.proc.name"),
        Some(&Value::from("sh"))
    );

    // A scalar under a named key is not an empty map and is left alone.
    let mut scalar = Event::new(serde_json::json!({ "sysdig": { "event": { "content": {
        "fields": { "proc": { "pid": 1_890_726 } }
    } } } }));
    assert!(remove_empty_child_maps(&mut scalar, &pattern));
    assert_eq!(
        scalar.get("sysdig.event.content.fields.proc.pid"),
        Some(&Value::from(1_890_726))
    );
}

/// gdacs's composite `event.id`, and the reason the accumulator's NAME is
/// not the trigger: `parts` is gdacs's spelling and nothing else's.
#[test]
fn a_composite_key_joins_only_the_fields_that_are_there() {
    let script = "if (ctx.event == null) { ctx.event = new HashMap(); }\n\
        def parts = new ArrayList();\n\
        if (ctx.gdacs?.event_id != null) { parts.add(ctx.gdacs.event_id.toString()); }\n\
        if (ctx.gdacs?.episode_id != null) { parts.add(ctx.gdacs.episode_id.toString()); }\n\
        if (ctx.gdacs?.geometry_id != null) { parts.add(ctx.gdacs.geometry_id.toString()); }\n\
        if (parts.size() > 0) {\n  ctx.event.id = String.join(\"-\", parts);\n}\n";
    let pattern = parse_join_present_fields(&normalise(script)).expect("gdacs builds an id");
    assert_eq!(
        pattern.sources,
        ["gdacs.event_id", "gdacs.episode_id", "gdacs.geometry_id"]
    );
    assert_eq!(pattern.separator, "-");
    assert_eq!(pattern.target, "event.id");

    // An absent middle id contributes NOTHING, not an empty segment.
    let mut event = Event::new(serde_json::json!({
        "gdacs": { "event_id": 1234, "geometry_id": "G9" }
    }));
    assert!(join_present_fields(&mut event, &pattern));
    assert_eq!(event.get("event.id"), Some(&Value::from("1234-G9")));
}

/// gdacs grades an alert level, and both defects it exposed are here: the
/// local narrows ITSELF (`level = level.toLowerCase()`), which the
/// declaration scan cannot see, and each arm writes TWO fields.
#[test]
fn a_ladder_folds_a_self_narrowed_local_and_keeps_every_write() {
    let script = "def level = ctx.gdacs?.alert_level;\nif (level == null) { return; }\n\
        level = level.toLowerCase();\nif (level == \"red\") {\n  ctx.event.severity = 3;\n  \
        ctx.event.risk_score = 90.0;\n} else if (level == \"orange\") {\n  \
        ctx.event.severity = 2;\n  ctx.event.risk_score = 60.0;\n}\n";
    let ladder = parse_ladder(&normalise(script)).expect("gdacs grades an alert level");
    assert_eq!(ladder.subject, "gdacs.alert_level");
    assert!(ladder.fold_case, "the self-fold travels with the subject");
    assert_eq!(
        ladder.arms[0].writes,
        vec![
            ("event.severity".to_owned(), Value::from(3)),
            ("event.risk_score".to_owned(), Value::from(90.0)),
        ]
    );

    // The EVENT carries the unfolded spelling; the fold is what matches it.
    let mut event = Event::new(serde_json::json!({"gdacs": {"alert_level": "Red"}}));
    assert!(try_ladder(&mut event, &ladder));
    assert_eq!(event.get("event.severity"), Some(&Value::from(3)));
    assert_eq!(event.get("event.risk_score"), Some(&Value::from(90.0)));
}

/// gdacs's disaster-code table, written inline rather than in `params`.
#[test]
fn a_local_map_literal_is_a_lookup_table() {
    let script = "def typeMap = [\n  'EQ': 'Earthquake',\n  'TC': 'Tropical Cyclone',\n  \
        'FL': 'Flood'\n];\ndef code = ctx.gdacs?.event_type;\n\
        if (code != null && typeMap.containsKey(code)) {\n  \
        ctx.gdacs.event_type_name = typeMap[code];\n}\n";
    let pattern = parse_local_map_lookup(&normalise(script)).expect("the table parses");
    assert_eq!(pattern.source, "gdacs.event_type");
    assert_eq!(pattern.target, "gdacs.event_type_name");
    assert_eq!(
        pattern.table.get("TC").map(String::as_str),
        Some("Tropical Cyclone")
    );

    let named = |code: &str| {
        let mut event = Event::new(serde_json::json!({ "gdacs": { "event_type": code } }));
        run_local_map_lookup(&mut event, &pattern);
        event
            .get("gdacs.event_type_name")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string)
    };
    assert_eq!(named("EQ").as_deref(), Some("Earthquake"));
    // A value with a space survives its quotes.
    assert_eq!(named("TC").as_deref(), Some("Tropical Cyclone"));
    // Unlisted: the script's own containsKey guard writes nothing.
    assert_eq!(named("ZZ"), None);
}

/// watchguard's own KV scanner, on the patterns its logs actually carry.
///
/// A quoted value holding spaces is the whole reason the vendor wrote a
/// scanner instead of using the kv processor, and `msg` is the field it
/// exists for.
#[test]
fn a_quoted_kv_scan_keeps_spaces_inside_quotes() {
    let pattern = QuotedKvScan {
        source: "_temp".to_string(),
        target: "watchguard_firebox.log".to_string(),
    };
    let mut event = Event::new(serde_json::json!({
        "_temp": "app_id=63 app_name=\"World WideWeb HTTP\" \
            msg=\"Application identified\" sig_vers=18.123",
    }));
    assert!(run_quoted_kv_scan(&mut event, &pattern));

    let at = |key: &str| {
        event
            .get(&format!("watchguard_firebox.log.{key}"))
            .and_then(serde_json::Value::as_str)
            .map(str::to_string)
    };
    assert_eq!(at("app_id").as_deref(), Some("63"));
    assert_eq!(
        at("app_name").as_deref(),
        Some("World WideWeb HTTP"),
        "a quoted value keeps its spaces and loses its quotes"
    );
    assert_eq!(at("msg").as_deref(), Some("Application identified"));
    assert_eq!(at("sig_vers").as_deref(), Some("18.123"));
}

/// Google Workspace's parameter fan-out, verbatim from
/// `google_workspace_drive`, which four sibling streams also carry.
#[test]
fn a_parameter_list_fans_out_by_name() {
    let script = "if (ctx.google_workspace.drive == null) {\n  \
        ctx.google_workspace.drive = new HashMap();\n} \
        for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  \
        if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"value\"] != null) {\n    \
        ctx.google_workspace.drive[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = \
        ctx[\"json\"][\"events\"][\"parameters\"][i][\"value\"];\n  }\n  \
        if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiValue\"] != null) {\n    \
        ctx.google_workspace.drive[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = \
        ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiValue\"];\n  }\n  \
        if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"boolValue\"] != null) {\n    \
        ctx.google_workspace.drive[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = \
        ctx[\"json\"][\"events\"][\"parameters\"][i][\"boolValue\"];\n  }\n}\n";

    let pattern = parse_parameter_fan_out(&normalise(script)).expect("the fan-out parses");
    assert_eq!(pattern.list, "json.events.parameters");
    assert_eq!(pattern.target, "google_workspace.drive");
    assert_eq!(pattern.values, ["value", "multiValue", "boolValue"]);

    let mut event = Event::new(serde_json::json!({
        "json": { "events": { "parameters": [
            { "name": "doc_title", "value": "document title" },
            { "name": "billable", "boolValue": false },
            { "name": "owners", "multiValue": ["a@example.com", "b@example.com"] },
            { "name": "nothing", "value": null },
        ] } },
    }));
    assert!(run_parameter_fan_out(&mut event, &pattern));

    assert_eq!(
        event.get("google_workspace.drive.doc_title"),
        Some(&serde_json::json!("document title"))
    );
    // A false is a VALUE, not an absence: reading it as one dropped every
    // `billable: false` the corpus expects.
    assert_eq!(
        event.get("google_workspace.drive.billable"),
        Some(&serde_json::json!(false))
    );
    assert_eq!(
        event.get("google_workspace.drive.owners"),
        Some(&serde_json::json!(["a@example.com", "b@example.com"]))
    );
    assert_eq!(
        event.get("google_workspace.drive.nothing"),
        None,
        "an explicit null is what the script's own guard skips"
    );
}

/// A map merged into another with no loop at all.
///
/// tanium's `threat_response` lands its whole `Match Details` payload
/// through this one call. The subject is the LAST statement's: the target
/// is allocated on the line above, and reading from the start of the
/// script took both statements for one path.
///
/// The generated call site is inside `if false` -- its condition,
/// `ctx.json['Match Details'] instanceof Map`, is one of the 282
/// `skipped_processors.rs` counts -- so the corpus cannot exercise this
/// yet and the unit test is what holds it.
#[test]
fn a_put_all_merges_a_map_into_another() {
    let script = "ctx.tanium.threat_response.match_details = \
        ctx.tanium.threat_response.match_details ?: [:];\n\
        ctx.tanium.threat_response.match_details.putAll(ctx.json['Match Details']);\n";
    let pattern = parse_put_all(&normalise(script)).expect("the merge parses");
    assert_eq!(pattern.parent, "tanium.threat_response.match_details");
    assert_eq!(pattern.source, "json.Match Details");
    assert!(
        !pattern.take,
        "a putAll reads the source, it does not consume it"
    );

    let mut event = Event::new(serde_json::json!({
        "json": { "Match Details": { "finding": { "id": "5212345" }, "config_id": 1_000_111 } },
        "tanium": { "threat_response": { "match_details": { "kept": true } } },
    }));
    assert!(run_merge_map_up(&mut event, &pattern));
    assert_eq!(
        event.get("tanium.threat_response.match_details.finding.id"),
        Some(&serde_json::json!("5212345"))
    );
    assert_eq!(
        event.get("tanium.threat_response.match_details.kept"),
        Some(&serde_json::json!(true)),
        "what the target already held survives the merge"
    );
}

/// The arm forms `pipelines/kolide/audit/extended-mappings.yml` uses,
/// verbatim, around its own opening and closing lines.
const KOLIDE_CLASSIFY: &str = "String m = ctx.message;\n\
    String a = null;\n\
    String c = null;\n\
    \n\
    // Exact-match, fixed-description events (no extractable fields; no grok).\n\
    if (m == 'Created an API key') { a = 'api_key_created'; }\n\
    else if (m == \"modified the organization's extended device compliance configuration\") \
    { a = 'extended_device_compliance_configuration_changed'; }\n\
    \n\
    // Checks.\n\
    else if (m.startsWith('Deleted Check ')) { a = 'check_deleted'; c = 'check'; }\n\
    else if (m.startsWith('Changed Fix Instructions Template ') || \
    m.startsWith('Changed Rationale Template ')) { a = 'check_configuration_changed'; c = 'check'; }\n\
    else if (m.startsWith('Reopened previously ') && m.contains(' device registration for ')) \
    { a = 'device_registration_reopened'; c = 'device_reg'; }\n\
    else if (m.contains(' registration self-approved by ')) \
    { a = 'device_registration_self_approved'; c = 'device_reg'; }\n\
    else if (m.startsWith('Device ') && m.endsWith(' removed')) \
    { a = 'device_removed'; c = 'device_removal'; }\n\
    else if (m.startsWith(\"Changed '\")) { c = 'setting_generic'; }\n\
    \n\
    if (a != null) {\n\
      if (ctx.event == null) { ctx.event = new HashMap(); }\n\
      ctx.event.action = a;\n\
    }\n\
    if (c != null) {\n\
      if (ctx._tmp == null) { ctx._tmp = new HashMap(); }\n\
      ctx._tmp.cat = c;\n\
    }";

/// `pipelines/ti_abusech/url/default.yml`, which five of the six streams
/// ship: the duration is configured, the base is a pair of candidates.
const EXPIRY_FROM_SEEN: &str = "def dur = (ctx._conf?.ioc_expiration_duration != null && \
    ctx._conf.ioc_expiration_duration instanceof String && \
    ctx._conf.ioc_expiration_duration != '') ? ctx._conf.ioc_expiration_duration : '90d';\n\
    ZonedDateTime _tmp_deleted_at;\n\
    ZonedDateTime _tmp_updated_at;\n\
    if (ctx.threat.indicator.last_seen != null) {\n\
      _tmp_updated_at = ZonedDateTime.parse(ctx.threat.indicator.last_seen);\n\
    } else {\n\
      _tmp_updated_at = ZonedDateTime.parse(ctx.threat.indicator.first_seen);\n\
    }\n\
    String time_unit = dur.substring(dur.length() -  1, dur.length());\n\
    String time_value = dur.substring(0, dur.length() - 1);\n\
    if (time_unit == 'd') {\n\
      _tmp_deleted_at = _tmp_updated_at.plusDays(Long.parseLong(time_value));\n\
    } else if (time_unit == 'h') {\n\
      _tmp_deleted_at = _tmp_updated_at.plusHours(Long.parseLong(time_value));\n\
    } else if (time_unit == 'm') {\n\
      _tmp_deleted_at = _tmp_updated_at.plusMinutes(Long.parseLong(time_value));\n\
    } else {\n\
      _tmp_deleted_at = _tmp_updated_at.plusDays(90L);\n\
      if (ctx.error == null) { ctx.error = new HashMap(); }\n\
      if (ctx.error.message == null) { ctx.error.message = new ArrayList(); }\n\
      ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n\
    }\n\
    ctx.abusech.url.deleted_at = _tmp_deleted_at;\n";

/// `pipelines/ti_abusech/ja3_fingerprints/default.yml`: the base is bound
/// straight off `ctx`, and the result is pulled back so an indicator
/// expires just before the next interval.
const EXPIRY_FROM_INGEST: &str = "def dur = ctx.labels.interval;\n\
    ZonedDateTime _tmp_deleted_at;\n\
    ZonedDateTime _tmp_created_at = ctx.event.ingested;\n\
    long max_ingest_time_in_sec = 30L;\n\
    long transform_max_age_in_min = 1L;\n\
    String time_unit = dur.substring(dur.length() -  1, dur.length());\n\
    String time_value = dur.substring(0, dur.length() - 1);\n\
    if (time_unit == 'd') {\n\
      _tmp_deleted_at = _tmp_created_at.plusDays(Long.parseLong(time_value));\n\
    } else if (time_unit == 'h') {\n\
      _tmp_deleted_at = _tmp_created_at.plusHours(Long.parseLong(time_value));\n\
    } else if (time_unit == 'm') {\n\
      _tmp_deleted_at = _tmp_created_at.plusMinutes(Long.parseLong(time_value));\n\
    } else {\n\
      _tmp_deleted_at = _tmp_created_at.plusDays(90L);\n\
      if (ctx.error == null) { ctx.error = new HashMap(); }\n\
      if (ctx.error.message == null) { ctx.error.message = new ArrayList(); }\n\
      ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n\
    }\n\
    _tmp_deleted_at = _tmp_deleted_at.minusMinutes(transform_max_age_in_min)\
    .minusSeconds(max_ingest_time_in_sec);\n\
    ctx.abusech.ja3_fingerprints.deleted_at = _tmp_deleted_at;\n";

// Both constants are the FOLDED form the call sites actually hold: these
// scripts are YAML `source: >`, so the preamble is one line and only the
// braces carry newlines. A pretty-printed copy parses differently and
// would test a script that never ships.

/// `pipelines/ti_anomali/threatstream/default.yml`: each arm writes a
/// bracketed ctx key and returns, and the out-of-range band is an `||`.
const CONFIDENCE_RETURNING: &str = "def value = ctx.json.confidence; \
    if (value <= 0.0 || value > 100.0) {\n  \
    ctx[\"threatintel_indicator_confidence\"] = \"None\";\n  return;\n} \
    if (value >= 1.0 && value <= 29.0) {\n  \
    ctx[\"threatintel_indicator_confidence\"] = \"Low\";\n  return;\n} \
    if (value >= 30.0 && value <= 69.0) {\n  \
    ctx[\"threatintel_indicator_confidence\"] = \"Medium\";\n  return;\n} \
    if (value >= 70 && value <= 100) {\n  \
    ctx[\"threatintel_indicator_confidence\"] = \"High\";\n  return;\n}\n";

/// `pipelines/ti_abusech/threatfox/default.yml`: the arms fill a local one
/// closing `.put()` writes, and the local's initialiser is the default.
const CONFIDENCE_PUT: &str = "def value = ctx.abusech.threatfox.confidence_level; \
    def confidence = \"None\"; if (value > 0 && value < 30) {\n  confidence = \"Low\";\n\
    } if (value >= 30.0 && value < 70) {\n  confidence = \"Medium\";\n\
    } else if (value >= 70 && value <= 100) {\n  confidence = \"High\";\n\
    } ctx.threat.indicator.put(\"confidence\", confidence)\n";

/// `pipelines/ti_anomali/threatstream/default.yml`: one target, a
/// different source per label, each arm carrying its own null guard.
const NAME_BY_TYPE: &str = "String indicatorType = ctx.threat?.indicator?.type;\n\
    if (indicatorType == 'ipv4-addr' || indicatorType == 'ipv6-addr') {\n  \
    if (ctx.json?.srcip != null) ctx.threat.indicator.name = ctx.json.srcip;\n\
    } else if (indicatorType == 'domain-name') {\n  \
    if (ctx.json?.domain != null) ctx.threat.indicator.name = ctx.json.domain;\n\
    } else if (indicatorType == 'url') {\n  \
    if (ctx.json?.url != null) ctx.threat.indicator.name = ctx.json.url;\n\
    } else if (indicatorType == 'file') {\n  \
    if (ctx.json?.md5 != null) ctx.threat.indicator.name = ctx.json.md5;\n}\n";

/// The same split as the `new ArrayList()` spelling, written as a stream.
const SPLIT_STREAM: &str = "def lst = Stream.of(ctx.json.trusted_circle_ids\
    .splitOnToken(',')).filter(s -> !s.isEmpty()).collect(Collectors.toList()); \
    if (lst.size() > 0) {\n  ctx.json.trusted_circle_ids = lst;\n\
    } else {\n  ctx.json.remove('trusted_circle_ids');\n}\n";

/// Each label takes its OWN source. Reading the first arm's source for
/// every label is what the guarded-copy fallback did.
/// `pipelines/ti_anomali/intelligence/default.yml`, the THIRD spelling of
/// the confidence band: two map-creation guards before the ladder, a
/// `== null` arm, one bound written literal-first (`100.0 < value`), and a
/// bare `else` for the top band.
const CONFIDENCE_WITH_ELSE: &str = "if (ctx.threat == null) {\n  ctx.threat = [:];\n\
    } if (ctx.threat.indicator == null) {\n  ctx.threat.indicator = [:];\n\
    } def value = ctx.json.confidence; if (value == null) {\n  \
    ctx.threat.indicator.confidence = \"Not Specified\";\n\
    } else if (value <= 0.0 || 100.0 < value) {\n  \
    ctx.threat.indicator.confidence = \"None\";\n\
    } else if (value < 30.0) {\n  ctx.threat.indicator.confidence = \"Low\";\n\
    } else if (value < 70.0) {\n  ctx.threat.indicator.confidence = \"Medium\";\n\
    } else {\n  ctx.threat.indicator.confidence = \"High\";\n}\n";

/// Every reading the third spelling needs at once: the null arm, a bound
/// the vendor wrote the other way round, and the bare `else`.
#[test]
fn a_band_ladder_reads_a_null_arm_a_mirrored_bound_and_a_bare_else() {
    for (confidence, expected) in [
        (json!(-1), "None"),
        (json!(0), "None"),
        (json!(150), "None"),
        (json!(1), "Low"),
        (json!(29), "Low"),
        (json!(30), "Medium"),
        (json!(69), "Medium"),
        (json!(70), "High"),
        (json!(100), "High"),
    ] {
        let mut event = Event::new(json!({ "json": { "confidence": confidence } }));
        assert!(try_known_painless(&mut event, CONFIDENCE_WITH_ELSE));
        assert_eq!(
            event.get_str("threat.indicator.confidence"),
            Some(expected),
            "confidence {confidence}"
        );
    }

    // The `== null` arm: an absent subject gets its OWN label, which is a
    // different thing from a present value in no band.
    let mut absent = Event::new(json!({ "json": {} }));
    assert!(try_known_painless(&mut absent, CONFIDENCE_WITH_ELSE));
    assert_eq!(
        absent.get_str("threat.indicator.confidence"),
        Some("Not Specified")
    );
}

/// `pipelines/ti_abusech/malwarebazaar/default.yml`: the hash map's values
/// gathered into `related.hash`, a value that is itself a list flattened
/// in, and nothing repeated.
#[test]
fn a_map_is_flattened_into_a_deduped_list() {
    let mut event = Event::new(json!({
        "threat": { "indicator": { "file": { "hash": {
            "md5": "aaa",
            "sha256": "bbb",
            // ssdeep arrives as a list, and one member repeats the md5.
            "ssdeep": ["ccc", "ddd", "aaa"],
        } } } },
    }));
    assert!(try_known_painless(&mut event, COLLECT_HASHES));
    assert_eq!(
        event.get("related.hash"),
        Some(&json!(["aaa", "bbb", "ccc", "ddd"]))
    );
}

/// The script APPENDS -- an earlier processor may have put a hash there,
/// and replacing the list would drop it.
#[test]
fn flattening_keeps_what_the_target_already_held() {
    let mut event = Event::new(json!({
        "related": { "hash": ["existing"] },
        "threat": { "indicator": { "file": { "hash": { "md5": "aaa" } } } },
    }));
    assert!(try_known_painless(&mut event, COLLECT_HASHES));
    assert_eq!(event.get("related.hash"), Some(&json!(["existing", "aaa"])));
}

const COLLECT_HASHES: &str = "def map = ctx.threat.indicator.file.hash;\n\
    if(ctx.related == null) {\n    ctx.put('related', new HashMap());\n}\n\
    if(ctx.related.hash == null) {\n    ctx.related.put('hash',new ArrayList());\n}\n\
    for (def x : map.values()) {\n    if (x instanceof List) {\n        \
    for (def i : x) {\n            if(i != null && !ctx.related.hash.contains(i)) {\n\
    ctx.related.hash.add(i);\n            }\n        }\n    \
    } else if(x != null && !ctx.related.hash.contains(x)) {\n        \
    ctx.related.hash.add(x);\n    }\n}\n";

/// A big id arrives from JSON as a number and Elasticsearch renders it as
/// a string; leaving it numeric is a type mismatch on every event.
#[test]
fn longs_are_stringified_where_they_are_not_already_strings() {
    let script = "if (ctx.json?.id != null && !(ctx.json.id instanceof String)) {\n  \
        ctx.json.id = Long.toString((long) ctx.json.id);\n\
        }\nif (ctx.json?.update_id != null && !(ctx.json.update_id instanceof String)) {\n  \
        ctx.json.update_id = Long.toString((long) ctx.json.update_id);\n}\n";

    let mut event = Event::new(json!({
        "json": { "id": 185_029_228, "update_id": "376590230" },
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("json.id"), Some("185029228"));
    // Already a string -- the script's `instanceof String` guard.
    assert_eq!(event.get_str("json.update_id"), Some("376590230"));
}

#[test]
fn a_copy_ladder_takes_the_source_its_label_names() {
    let sources = json!({
        "srcip": "10.0.0.1",
        "domain": "example.test",
        "url": "https://example.test/a",
        "md5": "d41d8cd98f00b204e9800998ecf8427e",
    });
    for (indicator, expected) in [
        ("ipv4-addr", "10.0.0.1"),
        ("ipv6-addr", "10.0.0.1"),
        ("domain-name", "example.test"),
        ("url", "https://example.test/a"),
        ("file", "d41d8cd98f00b204e9800998ecf8427e"),
    ] {
        let mut event = Event::new(json!({
            "threat": { "indicator": { "type": indicator } },
            "json": sources,
        }));
        assert!(try_known_painless(&mut event, NAME_BY_TYPE), "{indicator}");
        assert_eq!(
            event.get_str("threat.indicator.name"),
            Some(expected),
            "type {indicator}"
        );
    }
}

/// A label with no arm, and an arm whose source is absent, both leave the
/// target alone -- the script's own `!= null` guards say so.
#[test]
fn a_copy_ladder_writes_nothing_it_was_not_given() {
    let mut unmatched = Event::new(json!({
        "threat": { "indicator": { "type": "x509-certificate" } },
        "json": { "srcip": "10.0.0.1" },
    }));
    assert!(try_known_painless(&mut unmatched, NAME_BY_TYPE));
    assert_eq!(unmatched.get("threat.indicator.name"), None);

    let mut absent = Event::new(json!({
        "threat": { "indicator": { "type": "url" } },
        "json": { "srcip": "10.0.0.1" },
    }));
    assert!(try_known_painless(&mut absent, NAME_BY_TYPE));
    assert_eq!(absent.get("threat.indicator.name"), None);
}

/// The stream filter drops EVERY empty piece, so a value fenced by its own
/// separator loses both ends -- Java's split keeps the leading one.
#[test]
fn a_stream_split_drops_every_empty_piece() {
    let mut event = Event::new(json!({
        "json": { "trusted_circle_ids": ",10015," },
    }));
    assert!(try_known_painless(&mut event, SPLIT_STREAM));
    assert_eq!(
        event.get("json.trusted_circle_ids"),
        Some(&json!(["10015"]))
    );
}

/// Nothing surviving the filter deletes the field, which is the script's
/// own `else` branch -- an empty array would be a value the vendor never
/// writes.
#[test]
fn a_stream_split_removes_a_field_with_no_pieces() {
    let mut event = Event::new(json!({
        "json": { "trusted_circle_ids": ",,,", "keep": "me" },
    }));
    assert!(try_known_painless(&mut event, SPLIT_STREAM));
    assert_eq!(event.get("json.trusted_circle_ids"), None);
    assert_eq!(event.get_str("json.keep"), Some("me"));
}

#[test]
fn a_returning_band_ladder_takes_the_first_band_that_holds() {
    for (confidence, expected) in [
        (0, Some("None")),
        (101, Some("None")),
        (1, Some("Low")),
        (29, Some("Low")),
        (30, Some("Medium")),
        (69, Some("Medium")),
        (70, Some("High")),
        (100, Some("High")),
    ] {
        let mut event = Event::new(json!({ "json": { "confidence": confidence } }));
        assert!(try_known_painless(&mut event, CONFIDENCE_RETURNING));
        assert_eq!(
            event.get_str("threatintel_indicator_confidence"),
            expected,
            "confidence {confidence}"
        );
    }
}

/// Between the bands the script returns having written nothing, and a
/// default would be this pattern inventing one.
#[test]
fn a_ladder_with_no_default_writes_nothing_between_bands() {
    let mut event = Event::new(json!({ "json": { "confidence": 0.5 } }));
    assert!(try_known_painless(&mut event, CONFIDENCE_RETURNING));
    assert_eq!(event.get("threatintel_indicator_confidence"), None);
}

/// The local's initialiser IS the no-band label, and the closing `.put()`
/// names where it lands.
#[test]
fn a_put_band_ladder_falls_back_to_the_locals_initialiser() {
    for (level, expected) in [
        (0, "None"),
        (150, "None"),
        (1, "Low"),
        (29, "Low"),
        (30, "Medium"),
        (69, "Medium"),
        (70, "High"),
        (100, "High"),
    ] {
        let mut event =
            Event::new(json!({ "abusech": { "threatfox": { "confidence_level": level } } }));
        assert!(try_known_painless(&mut event, CONFIDENCE_PUT));
        assert_eq!(
            event.get_str("threat.indicator.confidence"),
            Some(expected),
            "confidence_level {level}"
        );
    }
}

/// A vendor field that has not been through a `convert` is a string, and
/// banding it as one would put every event in the same arm.
#[test]
fn a_band_ladder_reads_a_numeric_string() {
    let mut event = Event::new(json!({ "json": { "confidence": "85" } }));
    assert!(try_known_painless(&mut event, CONFIDENCE_RETURNING));
    assert_eq!(
        event.get_str("threatintel_indicator_confidence"),
        Some("High")
    );
}

/// Verbatim from `testdata/compat/ti_abusech/url/test-abusechurl-dump`:
/// 2021-10-05 plus the configured 90 days.
#[test]
fn an_expiry_is_the_base_plus_the_configured_duration() {
    let mut event = Event::new(json!({
        "_conf": { "ioc_expiration_duration": "90d" },
        "threat": { "indicator": { "first_seen": "2021-10-05T13:57:05.000Z" } },
    }));
    assert!(try_known_painless(&mut event, EXPIRY_FROM_SEEN));
    assert_eq!(
        event.get_str("abusech.url.deleted_at"),
        Some("2022-01-03T13:57:05.000Z")
    );
    assert_eq!(event.get("error.message"), None);
}

/// The script tries `last_seen` first, so a document carrying both must
/// expire from the later one.
#[test]
fn last_seen_beats_first_seen() {
    let mut event = Event::new(json!({
        "_conf": { "ioc_expiration_duration": "2h" },
        "threat": { "indicator": {
            "first_seen": "2021-10-05T13:57:05.000Z",
            "last_seen": "2021-11-05T01:02:03.000Z",
        } },
    }));
    assert!(try_known_painless(&mut event, EXPIRY_FROM_SEEN));
    assert_eq!(
        event.get_str("abusech.url.deleted_at"),
        Some("2021-11-05T03:02:03.000Z")
    );
}

/// No duration configured is the version-upgrade case, and the script's own
/// ternary names the fallback. It is NOT the invalid-unit branch, so no
/// error is appended -- writing one on every event is what the partial walk
/// under `GuardedCopy` used to do.
#[test]
fn an_absent_duration_takes_the_literal_default_quietly() {
    let mut event = Event::new(json!({
        "threat": { "indicator": { "first_seen": "2021-10-05T13:57:05.000Z" } },
    }));
    assert!(try_known_painless(&mut event, EXPIRY_FROM_SEEN));
    assert_eq!(
        event.get_str("abusech.url.deleted_at"),
        Some("2022-01-03T13:57:05.000Z")
    );
    assert_eq!(event.get("error.message"), None);
}

/// A unit the script does not know falls to 90 days AND says so.
#[test]
fn an_unknown_unit_defaults_and_reports() {
    let mut event = Event::new(json!({
        "_conf": { "ioc_expiration_duration": "12w" },
        "threat": { "indicator": { "first_seen": "2021-10-05T13:57:05.000Z" } },
    }));
    assert!(try_known_painless(&mut event, EXPIRY_FROM_SEEN));
    assert_eq!(
        event.get_str("abusech.url.deleted_at"),
        Some("2022-01-03T13:57:05.000Z")
    );
    assert_eq!(
        event.get("error.message"),
        Some(&json!([
            "invalid ioc_expiration_duration: using default 90 days"
        ]))
    );
}

/// Verbatim from `testdata/compat/ti_abusech/ja3_fingerprints`: ingest time
/// plus the interval, less the minute and thirty seconds the script takes
/// back. The VALUE cannot be compared against Elasticsearch -- it is
/// derived from write time -- so this is where the arithmetic is checked.
#[test]
fn an_interval_expiry_settles_before_the_next_run() {
    let mut event = Event::new(json!({
        "labels": { "interval": "1h" },
        "event": { "ingested": "2026-08-30T11:55:06.583752823Z" },
    }));
    assert!(try_known_painless(&mut event, EXPIRY_FROM_INGEST));
    assert_eq!(
        event.get_str("abusech.ja3_fingerprints.deleted_at"),
        Some("2026-08-30T12:53:36.583Z")
    );
}

/// Every arm form the ladder uses lands the action AND the grok routing
/// key, which is what the blocks behind it are gated on.
#[test]
fn a_classify_ladder_writes_both_of_its_locals() {
    let cases = [
        ("Created an API key", Some("api_key_created"), None),
        (
            "modified the organization's extended device compliance configuration",
            Some("extended_device_compliance_configuration_changed"),
            None,
        ),
        (
            "Deleted Check \"Firewall\"",
            Some("check_deleted"),
            Some("check"),
        ),
        (
            "Changed Rationale Template Text for Check 'Firewall'",
            Some("check_configuration_changed"),
            Some("check"),
        ),
        (
            "Reopened previously denied device registration for \"a@b.c\".",
            Some("device_registration_reopened"),
            Some("device_reg"),
        ),
        (
            "Device \"host-1\" (ABC) registration self-approved by a@b.c from another trusted device",
            Some("device_registration_self_approved"),
            Some("device_reg"),
        ),
        (
            "Device 'host-1' (ABC) removed",
            Some("device_removed"),
            Some("device_removal"),
        ),
        (
            "Changed 'Prevent Deregistration' from 'false' to 'true'",
            None,
            Some("setting_generic"),
        ),
        ("something nothing matches", None, None),
    ];

    for (message, action, category) in cases {
        let mut event = Event::new(json!({ "message": message }));
        assert!(
            try_known_painless(&mut event, KOLIDE_CLASSIFY),
            "unclaimed: {message}"
        );
        assert_eq!(event.get_str("event.action"), action, "action: {message}");
        assert_eq!(event.get_str("_tmp.cat"), category, "cat: {message}");
    }
}

/// The first arm that holds claims the event, so a later arm whose test
/// also holds never runs.
#[test]
fn a_classify_ladder_stops_at_its_first_arm() {
    let mut event = Event::new(json!({
        "message": "Reopened previously denied device registration for \"a@b.c\" \
                    registration self-approved by a@b.c",
    }));
    assert!(try_known_painless(&mut event, KOLIDE_CLASSIFY));
    assert_eq!(
        event.get_str("event.action"),
        Some("device_registration_reopened")
    );
}

/// Verbatim from `pipelines/kolide/audit/default.yml`: the webhook
/// envelope moved out of the way of the ECS fields it collides with.
#[test]
fn the_webhook_envelope_moves_under_the_working_object() {
    let script = "if (ctx.containsKey('id')) { ctx.json.id = ctx.remove('id'); }\n\
        if (ctx.containsKey('timestamp')) { ctx.json.timestamp = ctx.remove('timestamp'); }\n\
        if (ctx.containsKey('data')) { ctx.json.data = ctx.remove('data'); }";

    let mut event = Event::new(json!({
        "id": "01JA67B1DYJCKJ1J73T0F5EWGR",
        "timestamp": "2024-10-14T19:16:05Z",
        "data": { "actor_name": "Alice" },
        "json": { "event": "audit_log.recorded" },
    }));
    assert!(try_known_painless(&mut event, script));

    assert_eq!(event.get("id"), None);
    assert_eq!(event.get("timestamp"), None);
    assert_eq!(event.get("data"), None);
    assert_eq!(event.get_str("json.id"), Some("01JA67B1DYJCKJ1J73T0F5EWGR"));
    assert_eq!(
        event.get_str("json.timestamp"),
        Some("2024-10-14T19:16:05Z")
    );
    assert_eq!(event.get_str("json.data.actor_name"), Some("Alice"));
}

/// Verbatim from `pipelines/kolide/osquery_status/default.yml`: one
/// address wrapped in the list ECS wants, under the guard Painless needs.
#[test]
fn a_list_literal_wraps_the_value_it_holds() {
    let script = "if (ctx.host == null) { ctx.host = new HashMap(); }\n\
        ctx.host.ip = [ ctx.json.kolide_decorations.remote_ip ];";

    let mut event = Event::new(json!({
        "json": { "kolide_decorations": { "remote_ip": "203.0.113.5" } },
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("host.ip"), Some(&json!(["203.0.113.5"])));
}

/// A wrap followed by a removal of its own source is ONE move, so the
/// pattern has to carry the removal rather than decline and drop it.
#[test]
fn a_wrap_that_removes_its_source_moves_the_value() {
    let script = "ctx.ocsf.resources = [ctx.ocsf.resource];\nctx.ocsf.remove('resource');";

    let mut event = Event::new(json!({ "ocsf": { "resource": { "uid": "r-1" } } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("ocsf.resources"),
        Some(&json!([{ "uid": "r-1" }]))
    );
    assert!(!event.has("ocsf.resource"));
}

/// Verbatim from `pipelines/kolide/auth/extended-mappings.yml`: the tail
/// of every sub-event description that carries the prefix.
#[test]
fn the_prefixed_descriptions_collect_their_tails() {
    let script = "def prefix = 'Downloaded package: ';\ndef packages = new ArrayList();\n\
        for (def ev : ctx.json.events) {\n  def desc = ev?.event_description;\n  \
        if (desc instanceof String && desc.startsWith(prefix)) {\n    \
        packages.add(desc.substring(prefix.length()));\n  }\n}\n\
        if (!packages.isEmpty()) {\n  \
        if (ctx.kolide.auth.downloaded_packages == null) {\n    \
        ctx.kolide.auth.downloaded_packages = packages;\n  } else {\n    \
        ctx.kolide.auth.downloaded_packages.addAll(packages);\n  }\n}";

    let mut event = Event::new(json!({ "json": { "events": [
        { "event_description": "Authentication succeeded" },
        { "event_description": "Downloaded package: linux-systemd-deb" },
        { "event_type": "no_description" },
    ]}}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("kolide.auth.downloaded_packages"),
        Some(&json!(["linux-systemd-deb"]))
    );

    // No description carries the prefix, so the field is not created.
    let mut bare = Event::new(json!({ "json": { "events": [
        { "event_description": "Authentication succeeded" },
    ]}}));
    assert!(try_known_painless(&mut bare, script));
    assert_eq!(bare.get("kolide.auth.downloaded_packages"), None);
}

/// Verbatim from `pipelines/kolide/audit/s3.yml`: the Log Pipeline
/// envelope's `data` emptied into the working object beside it.
#[test]
fn the_s3_envelope_data_is_lifted_and_the_routing_key_dropped() {
    let script = "Map data = (Map) ctx.json.remove('data');\n\
        for (def entry : data.entrySet()) {\n  \
        ctx.json[entry.getKey()] = entry.getValue();\n}\n\
        ctx.json.remove('type');";

    let mut event = Event::new(json!({ "json": {
        "type": "audit_log",
        "timestamp": "2026-06-04T19:10:55.090Z",
        "data": { "actor_name": "Noel Vasquez", "ip_address": "81.2.69.142" },
    }}));
    assert!(try_known_painless(&mut event, script));

    assert_eq!(event.get("json.data"), None);
    assert_eq!(event.get("json.type"), None);
    assert_eq!(event.get_str("json.actor_name"), Some("Noel Vasquez"));
    assert_eq!(event.get_str("json.ip_address"), Some("81.2.69.142"));
    assert_eq!(
        event.get_str("json.timestamp"),
        Some("2026-06-04T19:10:55.090Z")
    );
}

/// A key the envelope did not carry leaves its target alone.
#[test]
fn an_absent_envelope_key_is_not_moved() {
    let script = "if (ctx.containsKey('id')) { ctx.json.id = ctx.remove('id'); }";
    let mut event = Event::new(json!({ "json": { "event": "audit_log.recorded" } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("json.id"), None);
}

/// Verbatim from `pipelines/aws/cloudtrail/default.yml`, the prune half of
/// the pair. A map that held 31 entries and keeps 24 still renders through
/// a 64-bucket table, because Java's `HashMap` does not shrink on remove.
#[test]
fn a_prune_across_a_table_boundary_records_the_built_size() {
    let script = "void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    \
        if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        \
        handleList(v);\n    }\n    return v == null || v == '' || v == '-' || v == 'none' || \
        (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\n\
        void handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        \
        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    \
        return v == null || v == '' || v == '-' || v == 'none' || (v instanceof Map && v.size() == 0) || \
        (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n";

    // 13 entries down to 12 crosses the 16-to-32 boundary, and these
    // twelve keys render in a different order on either side of it.
    let mut inner = serde_json::Map::new();
    for key in [
        "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india",
        "juliet", "kilo", "lima",
    ] {
        inner.insert(key.to_string(), json!("v"));
    }
    inner.insert("gone".to_string(), Value::Null);
    let mut event = Event::new(json!({
        "json": { "responseElements": { "command": Value::Object(inner) } }
    }));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.map_capacity("json.responseElements.command"),
        Some(13)
    );
    assert_eq!(event.map_capacity("json"), None, "json lost nothing");

    // The render half of the pair reads that count back, so the members
    // walk the 32 buckets the map was built with rather than 16.
    let render = "if (ctx._conf.keep_flattened_duplicates && ctx.aws.cloudtrail?.flattened == null) \
        {\n  ctx.aws.cloudtrail.flattened = [:];\n}\nif (ctx.json?.responseElements != null) {\n  \
        ctx.aws.cloudtrail.response_elements = ctx.json.responseElements.toString();\n}\n";
    assert!(try_known_painless(&mut event, render));
    let rendered = event
        .get_str("aws.cloudtrail.response_elements")
        .unwrap_or_default()
        .to_string();
    let built = crate::painless_helpers::java_to_string_sized(
        event.get("json.responseElements").unwrap(),
        &mut "json.responseElements".to_string(),
        &|at| event.map_capacity(at),
    );
    assert_eq!(rendered, built, "the render did not read the capacity back");
    assert_ne!(
        rendered,
        crate::painless_helpers::java_to_string(event.get("json.responseElements").unwrap()),
        "a 32-bucket walk must differ from the 16-bucket one"
    );
}

/// Verbatim from `pipelines/netflow/log/default.yml`. The union is over
/// BOTH address families and the sort is lexicographic, so an IPv6 address
/// lands between two IPv4 ones.
#[test]
fn suffixed_keys_union_into_a_sorted_list() {
    let script = "HashSet set = null;\nfor (key in ctx.netflow.keySet()) {\n    \
        if (key.endsWith(\"_ipv4_address\") || key.endsWith(\"_ipv6_address\")) {\n        \
        if (set == null) {\n            set = new HashSet();\n        }\n        \
        set.add(ctx.netflow[key]);\n    }\n}\n\nif (set != null) {\n    \
        if (ctx.related == null) {\n        ctx.related = new HashMap();\n    }\n    \
        if (ctx.related?.ip != null) {\n        for (ip in ctx.related.ip) {\n            \
        set.add(ip);\n        }\n    }\n\n    ArrayList list = new ArrayList(set);\n    \
        Collections.sort(list);\n    ctx.related.ip = list;\n}\n";

    let mut event = Event::new(json!({
        "netflow": {
            "source_ipv6_address": "2a02:cf40::2",
            "destination_ipv6_address": "2a02:cf40::1",
            "protocol_identifier": 6
        },
        "related": { "ip": ["81.2.69.144", "0.0.0.0"] }
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("related.ip"),
        Some(&json!([
            "0.0.0.0",
            "2a02:cf40::1",
            "2a02:cf40::2",
            "81.2.69.144"
        ]))
    );
}

/// A map holding no key with a listed suffix is the script's `set == null`
/// branch, which leaves the target as it found it.
#[test]
fn no_suffixed_key_leaves_the_target_alone() {
    let script = "for (key in ctx.netflow.keySet()) {\n    \
        if (key.endsWith(\"_ipv4_address\")) {\n        set.add(ctx.netflow[key]);\n    }\n}\n\
        ArrayList list = new ArrayList(set);\nCollections.sort(list);\n\
        ctx.related.ip = list;\n";

    let mut event = Event::new(json!({
        "netflow": { "protocol_identifier": 6 },
        "related": { "ip": ["81.2.69.144"] }
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("related.ip"), Some(&json!(["81.2.69.144"])));
}

/// Verbatim from `pipelines/zscaler_zia/email_dlp/default.yml`: the vendor
/// ships its columns pipe-delimited in one string each.
#[test]
fn pipe_delimited_columns_split_in_place() {
    let script = "void splitStr(Map m, String key) {\n  if (m == null || key == null) return;\n  \
        def v = m.get(key);\n  if (!(v instanceof String)) return;\n  String s = (String) v;\n  \
        if (s.length() == 0) return;\n  List out = new ArrayList();\n  \
        out.add(s.substring(from, n));\n  m.put(key, out);\n}\n\
        def ed = ctx.zscaler_zia?.email_dlp;\nif (ed == null) return;\n\
        splitStr(ed, 'severity');\nif (ed.dlp instanceof Map) {\n  \
        splitStr(ed.dlp, 'dict_names');\n}\n";

    let mut event = Event::new(json!({ "zscaler_zia": { "email_dlp": {
        "severity": "a|b", "dlp": { "dict_names": "Credit Cards|SSN" }
    }}}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("zscaler_zia.email_dlp.severity"),
        Some(&json!(["a", "b"]))
    );
    assert_eq!(
        event.get("zscaler_zia.email_dlp.dlp.dict_names"),
        Some(&json!(["Credit Cards", "SSN"]))
    );
}

/// Verbatim from the same pipeline: names and counts zipped, and the
/// vendor's `None` placeholder dropping the whole entry.
#[test]
fn named_counts_zip_and_skip_the_placeholder() {
    let script = "def dlp = ctx.zscaler_zia.email_dlp.dlp;\n\
        def names = dlp.dict_names instanceof List ? dlp.dict_names : null;\n\
        def counts = dlp.dict_counts instanceof List ? dlp.dict_counts : null;\n\
        if (names == null) return;\ndef out = new ArrayList();\n\
        for (int i = 0; i < names.size(); i++) {\n  def name = names.get(i);\n  \
        if (!(name instanceof String) || name == '' || name == 'None') continue;\n  \
        def item = new HashMap();\n  item.put('name', name);\n  \
        if (counts != null && i < counts.size()) item.put('count', counts.get(i));\n  \
        out.add(item);\n}\n\
        if (out.size() > 0) ctx.zscaler_zia.email_dlp.dlp.dictionaries = out;\n";

    let mut event = Event::new(json!({ "zscaler_zia": { "email_dlp": { "dlp": {
        "dict_names": ["Credit Cards", "None", "SSN"], "dict_counts": [7, 0, 3]
    }}}}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("zscaler_zia.email_dlp.dlp.dictionaries"),
        Some(&json!([
            { "name": "Credit Cards", "count": 7 },
            { "name": "SSN", "count": 3 }
        ]))
    );
}

/// Verbatim from the same pipeline: parallel columns zipped one entry per
/// index, the md5 nested a level deeper than the rest.
#[test]
fn parallel_attachment_columns_zip_into_one_list() {
    let script = "def att = ctx.zscaler_zia.email_dlp.email.attachments;\n\
        def names = att.file_names instanceof List ? att.file_names : null;\n\
        def md5s = att.md5s instanceof List ? att.md5s : null;\n\
        def types = att.file_types instanceof List ? att.file_types : null;\n\
        int n = 0;\nif (names != null && names.size() > n) n = names.size();\n\
        if (n == 0) return;\nif (ctx.email == null) ctx.email = [:];\n\
        if (ctx.email.attachments == null) ctx.email.attachments = new ArrayList();\n\
        for (int i = 0; i < n; i++) {\n  def file = new HashMap();\n  \
        if (names != null && i < names.size()) file.put('name', names.get(i));\n  \
        if (md5s != null && i < md5s.size()) {\n    def hash = new HashMap();\n    \
        hash.put('md5', md5s.get(i));\n    file.put('hash', hash);\n  }\n  \
        if (types != null && i < types.size()) file.put('extension', types.get(i));\n  \
        def item = new HashMap();\n  item.put('file', file);\n  \
        ctx.email.attachments.add(item);\n}\n";

    let mut event = Event::new(json!({ "zscaler_zia": { "email_dlp": { "email": {
        "attachments": {
            "file_names": ["a.pdf"], "md5s": ["d41d8"], "file_types": ["pdf"]
        }
    }}}}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("email.attachments"),
        Some(&json!([{ "file": {
            "name": "a.pdf", "hash": { "md5": "d41d8" }, "extension": "pdf"
        }}]))
    );
}

/// Verbatim from `pipelines/aws/inspector/default.yml`: the ladder writes
/// through `.put()` and compares a local the script LOWER-CASED, so the
/// fold has to travel with the subject.
#[test]
fn a_case_folded_ladder_writes_through_put() {
    let script = "String severity = ctx.aws.inspector.severity.toLowerCase();\n\
        if (severity == 'untriaged') {\n  ctx.vulnerability.put('severity', 'Unknown');\n\
        } else if (severity == 'informational') {\n\
        ctx.vulnerability.put('severity', 'Low');\n\
        } else if (severity == 'high') {\n  ctx.vulnerability.put('severity', 'High');\n}";

    let mut event = Event::new(json!({ "aws": { "inspector": { "severity": "HIGH" }}}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("vulnerability.severity"), Some("High"));

    // A value no arm names writes nothing.
    let mut other = Event::new(json!({ "aws": { "inspector": { "severity": "none" }}}));
    assert!(try_known_painless(&mut other, script));
    assert!(other.get("vulnerability.severity").is_none());
}

/// Verbatim from `pipelines/mimecast/message_release_logs/default.yml`:
/// every display name and address the script names, split at the `@` and
/// sorted, with the operator object flattened to its address first.
#[test]
fn the_mail_related_sets_collect_and_sort() {
    let script = "def splitmail(String email) {\n\
        String[] parts = email.splitOnToken(\"@\");\n\
        if (parts.length != 2) {\n  return null;\n}\n  return parts;\n}\n\
        def users = new HashSet();\ndef hosts = new HashSet();\n\
        if (ctx.mimecast?.fromEnv?.displayableName != null) {\n\
        users.add(ctx.mimecast.fromEnv.displayableName);\n}\n\
        if (ctx.mimecast?.operator instanceof Map) {\n\
        ctx.mimecast.operator = ctx.mimecast.operator.emailAddress;\n}\n\
        if (ctx.mimecast?.operator != null) {\n\
        def parts = splitmail(ctx.mimecast.operator);\n\
        if (parts != null) {\n  users.add(parts[0]);\n  hosts.add(parts[1]);\n}\n\
        users.add(ctx.mimecast.operator);\n}\n\
        for (def to: ctx.mimecast.to) {\n\
        if (to.displayableName != null) {\n  users.add(to.displayableName);\n}\n\
        if (to.emailAddress != null) {\n\
        def parts = splitmail(to.emailAddress);\n\
        if (parts != null) {\n  users.add(parts[0]);\n  hosts.add(parts[1]);\n}\n\
        users.add(to.emailAddress);\n}\n}\n\
        if (users.size() != 0 || hosts.size() != 0) {\n\
        if (ctx.related == null) {\n  ctx.related = new HashMap();\n}\n\
        if (users.size() != 0 && ctx.related.user == null) {\n\
        ctx.related.user = new ArrayList();\n\
        for (def u: users) {\n  ctx.related.user.add(u);\n}\n\
        Collections.sort(ctx.related.user);\n}\n\
        if (hosts.size() != 0 && ctx.related.hosts == null) {\n\
        ctx.related.hosts = new ArrayList();\n\
        for (def h: hosts) {\n  ctx.related.hosts.add(h);\n}\n\
        Collections.sort(ctx.related.hosts);\n}\n}";

    let mut event = Event::new(json!({
        "mimecast": {
            "fromEnv": { "displayableName": "FromName LastName" },
            "operator": { "emailAddress": "admin@domain.tld" },
            "to": [{ "displayableName": "ToName LastName", "emailAddress": "to_user@to_domain.tld" }],
        },
    }));
    assert!(try_known_painless(&mut event, script));

    assert_eq!(
        event.get("mimecast.operator"),
        Some(&json!("admin@domain.tld"))
    );
    assert_eq!(
        event.get("related.user"),
        Some(&json!([
            "FromName LastName",
            "ToName LastName",
            "admin",
            "admin@domain.tld",
            "to_user",
            "to_user@to_domain.tld"
        ]))
    );
    assert_eq!(
        event.get("related.hosts"),
        Some(&json!(["domain.tld", "to_domain.tld"]))
    );
}

/// Verbatim from `pipelines/aws/cloudfront_logs/default.yml`: the split
/// x-forwarded-for is trimmed where it sits, because the grok that reads
/// each member next is anchored and a leading space fails it.
#[test]
fn a_split_list_is_trimmed_in_place() {
    let script = "for (int i = 0; i < ctx._tmp.split_x_forwarded_for.length; i++) {\n\
        ctx._tmp.split_x_forwarded_for[i] = ctx._tmp.split_x_forwarded_for[i].trim();\n}";
    let mut event = Event::new(json!({
        "_tmp": { "split_x_forwarded_for": ["81.2.69.142", " 216.160.83.56"] },
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("_tmp.split_x_forwarded_for"),
        Some(&json!(["81.2.69.142", "216.160.83.56"]))
    );
}

/// Its sibling: `localhost:8080` is not an address any grok reads, so the
/// script appends the loopback for each member that carries it.
#[test]
fn a_prefixed_member_appends_the_scripts_constant() {
    let script = "if (ctx.get('network') == null) {\n  ctx['network'] = new HashMap();\n}\n\
        for (String item : ctx._tmp.split_x_forwarded_for ) {\n\
        if (item.startsWith('localhost')) {\n\
        if (ctx.network.forwarded_ip == null) {\n\
        ctx['network']['forwarded_ip'] = new ArrayList();\n}\n\
        ctx['network']['forwarded_ip'].add('127.0.0.1');\n}\n}";

    let mut event = Event::new(json!({
        "_tmp": { "split_x_forwarded_for": ["localhost:8080"] },
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("network.forwarded_ip"),
        Some(&json!(["127.0.0.1"]))
    );

    // No member with the prefix leaves the list untouched.
    let mut plain = Event::new(json!({
        "_tmp": { "split_x_forwarded_for": ["81.2.69.142"] },
    }));
    assert!(try_known_painless(&mut plain, script));
    assert!(plain.get("network.forwarded_ip").is_none());
}

/// Verbatim from `pipelines/aws/cloudfront_logs/default.yml`: the URL is
/// reassembled out of whichever parts the log line carried.
#[test]
fn the_url_is_concatenated_from_the_parts_that_are_there() {
    let script = "def full = \"\";\n\
        if(ctx.network?.protocol != null && ctx.network?.protocol != \"\") {\n\
        full += ctx.network.protocol+\"://\";\n}\n\
        if(ctx.destination?.domain != null && ctx.destination?.domain != \"\") {\n\
        full += ctx.destination.domain;\n}\n\
        if(ctx.url?.path != null && ctx.url?.path != \"\") {\n  full += ctx.url.path;\n}\n\
        if(ctx.url?.query != null && ctx.url?.query != \"\") {\n\
        full += \"?\"+ctx.url.query;\n}\n\
        if(full != \"\") {\n  ctx._tmp.url_full = full\n}";

    let mut event = Event::new(json!({
        "network": { "protocol": "https" },
        "destination": { "domain": "test.com" },
        "url": { "path": "/getApplications", "query": "source=global" },
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("_tmp.url_full"),
        Some(&json!("https://test.com/getApplications?source=global"))
    );

    // A missing part takes its whole clause -- the `?` with the query.
    let mut partial = Event::new(json!({
        "network": { "protocol": "http" },
        "destination": { "domain": "www.example.com" },
        "url": { "path": "/" },
    }));
    assert!(try_known_painless(&mut partial, script));
    assert_eq!(
        partial.get("_tmp.url_full"),
        Some(&json!("http://www.example.com/"))
    );

    // Nothing to build from writes nothing.
    let mut empty = Event::new(json!({}));
    assert!(try_known_painless(&mut empty, script));
    assert!(empty.get("_tmp.url_full").is_none());
}

/// Verbatim from `pipelines/aws/elb_logs/default.yml`: the network load
/// balancer writes `tlsv12`, which is TLS 1.2.
#[test]
fn the_tls_token_splits_into_protocol_and_version() {
    let script = "def parts = ctx.aws.elb.ssl_protocol.splitOnToken(\"v\");\n\
        if (parts.length != 2) {\n  return;\n}\n\
        if (parts[1].contains(\".\")) {\n  ctx.tls.version = parts[1];\n} else {\n\
        ctx.tls.version = parts[1].substring(0,1) + \".\" + parts[1].substring(1);\n}\n\
        ctx.tls.version_protocol = parts[0].toLowerCase();";

    let mut event = Event::new(json!({ "aws": { "elb": { "ssl_protocol": "tlsv12" }}}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("tls.version"), Some(&json!("1.2")));
    assert_eq!(event.get("tls.version_protocol"), Some(&json!("tls")));

    // A token that does not split in two is the script's own early return.
    let mut plain = Event::new(json!({ "aws": { "elb": { "ssl_protocol": "-" }}}));
    assert!(try_known_painless(&mut plain, script));
    assert!(plain.get("tls.version").is_none());
}

/// Verbatim from `pipelines/aws/ec2_metrics/default.yml`: a `CloudWatch`
/// percentage becomes a fraction, but ONLY where the agent has not already
/// written the fraction itself. Read as a guarded copy, the second clause
/// of the guard was taken as the target and the raw 42 was written over
/// the agent's 0.421.
#[test]
fn a_percentage_divides_in_place_unless_the_agent_beat_it_to_it() {
    let script = "if(ctx.aws?.ec2?.metrics?.CPUUtilization?.avg != null \
         && ctx.host?.cpu?.usage == null) {\n    \
         ctx.aws.ec2.metrics.CPUUtilization.avg = \
         ctx.aws.ec2.metrics.CPUUtilization.avg / 100;\n}\n";

    // Firehose ships the percentage and nothing else.
    let mut firehose = Event::new(json!({
        "aws": {"ec2": {"metrics": {"CPUUtilization": {"avg": 21.96}}}}
    }));
    assert!(try_known_painless(&mut firehose, script));
    assert_eq!(
        firehose.get("aws.ec2.metrics.CPUUtilization.avg"),
        Some(&json!(21.96 / 100.0))
    );

    // The agent has already computed it, so the metric is left alone.
    let mut agent = Event::new(json!({
        "aws": {"ec2": {"metrics": {"CPUUtilization": {"avg": 42}}}},
        "host": {"cpu": {"usage": 0.421}},
    }));
    assert!(try_known_painless(&mut agent, script));
    assert_eq!(
        agent.get("aws.ec2.metrics.CPUUtilization.avg"),
        Some(&json!(42))
    );
    assert_eq!(agent.get("host.cpu.usage"), Some(&json!(0.421)));
}

/// Verbatim from `pipelines/carbonblack_edr/log/default.yml`, described
/// 'Selects a single document from docs input field'. Everything after it
/// reads `json.doc`, so leaving `json.docs` in place lost the lot.
#[test]
fn a_single_document_is_lifted_out_of_its_wrapper() {
    let script = "def docs = ctx.json.docs;\n\
         if (docs instanceof List && docs.size() > 0) {\n  \
         ctx.json[\"doc\"] = docs[0];\n\
         } else if (docs instanceof Map) {\n  ctx.json[\"doc\"] = docs;\n\
         } else {\n  throw new Exception(\"Unexpected type\");\n}\n\
         ctx.json.remove(\"docs\");";

    let mut listed = Event::new(json!({ "json": { "docs": [{ "pid": 44988 }] } }));
    assert!(try_known_painless(&mut listed, script));
    assert_eq!(listed.get("json.doc"), Some(&json!({ "pid": 44988 })));
    assert_eq!(listed.get("json.docs"), None);

    // The object form is taken as it stands, not wrapped and unwrapped.
    let mut single = Event::new(json!({ "json": { "docs": { "pid": 7 } } }));
    assert!(try_known_painless(&mut single, script));
    assert_eq!(single.get("json.doc"), Some(&json!({ "pid": 7 })));
    assert_eq!(single.get("json.docs"), None);

    // An empty list is the script's own `throw`: nothing is lifted, and
    // the source is NOT dropped, because the throw comes first.
    let mut empty = Event::new(json!({ "json": { "docs": [] } }));
    assert!(try_known_painless(&mut empty, script));
    assert_eq!(empty.get("json.doc"), None);
    assert_eq!(empty.get("json.docs"), Some(&json!([])));
}

/// Verbatim from `pipelines/cyberarkpas/audit/audit.yml`, tagged
/// `script_converts_caproperties_into_an_array_if_necessary`. The vendor
/// serialises one `CAProperty` as an object and several as a list, and the
/// `foreach` after this can only walk the list.
#[test]
fn a_lone_object_is_wrapped_so_the_foreach_can_walk_it() {
    let script = "def props = ctx.cyberarkpas?.audit?.CAProperties?.CAProperty;\n\
         if (props instanceof Map) {\n  \
         ctx.cyberarkpas.audit.CAProperties.CAProperty = [ props ];\n}\n";

    let mut single = Event::new(json!({ "cyberarkpas": { "audit": { "CAProperties": {
        "CAProperty": { "Name": "ConfigurationSchemaVersion", "Value": "12121" },
    }}}}));
    assert!(try_known_painless(&mut single, script));
    assert_eq!(
        single.get("cyberarkpas.audit.CAProperties.CAProperty"),
        Some(&json!([{ "Name": "ConfigurationSchemaVersion", "Value": "12121" }]))
    );

    // Already a list: left exactly as it stands, not nested a second time.
    let already = json!([{ "Name": "PolicyID", "Value": "LINUX-SSH" }]);
    let mut many = Event::new(json!({ "cyberarkpas": { "audit": { "CAProperties": {
        "CAProperty": already.clone(),
    }}}}));
    assert!(try_known_painless(&mut many, script));
    assert_eq!(
        many.get("cyberarkpas.audit.CAProperties.CAProperty"),
        Some(&already)
    );

    // Absent: the script's own null-safe navigation, so nothing is created.
    let mut absent = Event::new(json!({ "cyberarkpas": { "audit": {} } }));
    assert!(try_known_painless(&mut absent, script));
    assert_eq!(absent.get("cyberarkpas.audit.CAProperties"), None);
}

/// Verbatim from `pipelines/cyberarkpas/monitor/default.yml`, tagged
/// `script_set_host_cpu_usage`. The same divide, but the quotient lands on
/// a DIFFERENT field, the divisor is spelled `100.0`, and there is no space
/// either side of the slash.
#[test]
fn a_percentage_divides_into_a_different_field() {
    let script = "if (ctx.host == null) ctx.host = [:]; \
         if (ctx.host.cpu == null) ctx.host.cpu = [:]; \
         ctx.host.cpu.usage = ctx.cyberarkpas.monitor.cpu_usage/100.0;";

    let mut event = Event::new(json!({ "cyberarkpas": { "monitor": { "cpu_usage": 12 } } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("host.cpu.usage"), Some(&json!(0.12)));
    // The dividend is the vendor's own field and is left as it stands.
    assert_eq!(event.get("cyberarkpas.monitor.cpu_usage"), Some(&json!(12)));

    // `if (ctx.host == null)` is map CREATION, not a guard -- an event
    // that already carries a host must still get its usage.
    let mut with_host = Event::new(json!({
        "cyberarkpas": { "monitor": { "cpu_usage": 50 } },
        "host": { "name": "vault01" },
    }));
    assert!(try_known_painless(&mut with_host, script));
    assert_eq!(with_host.get("host.cpu.usage"), Some(&json!(0.5)));
}

/// Verbatim from `pipelines/aws/cloudtrail/default.yml`. `MobileVersion`
/// and `MFAUsed` are read as `!= 'No'`, so the string becomes a boolean;
/// `LoginTo` is carried as it stands.
#[test]
fn console_login_extras_become_two_booleans_and_a_url() {
    let script = "if (ctx.json?.eventName != 'ConsoleLogin') {\n  return;\n}\n\
         Map aed_map = [:];\n\
         if (ctx.json?.additionalEventData?.MobileVersion != null) {\n  \
         aed_map.mobile_version = ctx.json.additionalEventData.MobileVersion != 'No';\n}\n\
         if (ctx.json?.additionalEventData?.LoginTo != null) {\n  \
         aed_map.login_to = ctx.json.additionalEventData.LoginTo;\n}\n\
         if (ctx.json?.additionalEventData?.MFAUsed != null) {\n  \
         aed_map.mfa_used = ctx.json.additionalEventData.MFAUsed != 'No';\n}\n\
         if (aed_map.size() > 0) {\n  \
         ctx.aws.cloudtrail.console_login = [:];\n  \
         ctx.aws.cloudtrail.console_login.additional_eventdata = aed_map;\n}";

    let mut event = Event::new(json!({"json": {
        "eventName": "ConsoleLogin",
        "additionalEventData": {
            "MobileVersion": "No",
            "LoginTo": "https://console.aws.amazon.com/s3/",
            "MFAUsed": "No",
        },
    }}));
    assert!(try_known_painless(&mut event, script));
    let base = "aws.cloudtrail.console_login.additional_eventdata";
    assert_eq!(
        event.get(&format!("{base}.mobile_version")),
        Some(&json!(false))
    );
    assert_eq!(event.get(&format!("{base}.mfa_used")), Some(&json!(false)));
    assert_eq!(
        event.get_str(&format!("{base}.login_to")),
        Some("https://console.aws.amazon.com/s3/")
    );

    // Anything that is not the literal `No` is true, and another event
    // name writes nothing at all.
    let mut yes = Event::new(json!({"json": {
        "eventName": "ConsoleLogin",
        "additionalEventData": {"MFAUsed": "Yes"},
    }}));
    assert!(try_known_painless(&mut yes, script));
    assert_eq!(yes.get(&format!("{base}.mfa_used")), Some(&json!(true)));

    let mut other = Event::new(json!({"json": {
        "eventName": "AssumeRole",
        "additionalEventData": {"MFAUsed": "Yes"},
    }}));
    assert!(try_known_painless(&mut other, script));
    assert!(!other.has("aws.cloudtrail.console_login"));
}

/// Verbatim from `pipelines/m365_defender/event/pipeline_device.yml`: the
/// API name is whatever comes before the `ApiCall` marker, found without
/// regard to case but cut out of the ORIGINAL text.
#[test]
fn the_text_before_a_case_insensitive_marker_is_taken() {
    let script = "String actiontype = ctx.m365_defender.event.action.type;\n\
         def idx = actiontype.toLowerCase().lastIndexOf('apicall');\n\
         ctx._temp_process_Ext_api_name = actiontype.substring(0, idx);\n";

    let mut event = Event::new(json!({
        "m365_defender": {"event": {"action": {"type": "ReadProcessMemoryApiCall"}}}
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get_str("_temp_process_Ext_api_name"),
        Some("ReadProcessMemory")
    );

    // No marker means `substring(0, -1)`, which throws in Java, so
    // nothing is written and the rename behind it finds no field.
    let mut plain = Event::new(json!({
        "m365_defender": {"event": {"action": {"type": "SomethingElse"}}}
    }));
    assert!(try_known_painless(&mut plain, script));
    assert!(!plain.has("_temp_process_Ext_api_name"));
}

/// Verbatim from `pipelines/proofpoint_on_demand/message/default.yml`. A
/// fractional second becomes nanoseconds and closes the span the start
/// opened, rendered the way Java prints a `ZonedDateTime` -- three, six or
/// nine fractional digits, never a partial group.
#[test]
fn a_fractional_second_closes_the_span_it_opens() {
    let script = "ctx.event.duration = (int) (ctx.proofpoint_on_demand.message.filter\
         .duration_secs * 1000000000);\nif (ctx.event?.start != null) {\n  \
         ZonedDateTime start = ZonedDateTime.parse(ctx.event.start);\n  \
         ctx.event.end = start.plus(ctx.event.duration, ChronoUnit.NANOS);\n}\n";

    let mut event = Event::new(json!({
        "event": { "start": "2020-02-07T16:34:49.929Z" },
        "proofpoint_on_demand": {"message": {"filter": {"duration_secs": 0.286_712}}},
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("event.duration"), Some(&json!(286_712_000)));
    assert_eq!(
        event.get_str("event.end"),
        Some("2020-02-07T16:34:50.215712Z")
    );

    // Landing on a whole second prints NO fraction, which is Java's own
    // rendering and not a rounding of ours.
    let mut whole = Event::new(json!({
        "event": { "start": "2020-02-07T16:34:49.929Z" },
        "proofpoint_on_demand": {"message": {"filter": {"duration_secs": 0.071}}},
    }));
    assert!(try_known_painless(&mut whole, script));
    assert_eq!(whole.get_str("event.end"), Some("2020-02-07T16:34:50Z"));

    // A whole number of milliseconds prints three digits, not six.
    let mut millis = Event::new(json!({
        "event": { "start": "2020-02-07T16:34:49.929Z" },
        "proofpoint_on_demand": {"message": {"filter": {"duration_secs": 0.5}}},
    }));
    assert!(try_known_painless(&mut millis, script));
    assert_eq!(
        millis.get_str("event.end"),
        Some("2020-02-07T16:34:50.429Z")
    );

    // No start means no span, and the duration is still written.
    let mut startless = Event::new(json!({
        "proofpoint_on_demand": {"message": {"filter": {"duration_secs": 1.5}}},
    }));
    assert!(try_known_painless(&mut startless, script));
    assert_eq!(startless.get("event.duration"), Some(&json!(1_500_000_000)));
    assert!(!startless.has("event.end"));
}

/// Verbatim from `pipelines/m365_defender/vulnerability/default.yml`. The
/// defender exports disagree with themselves about casing, and every
/// `camelCase` event produced nothing at all while this went unmatched --
/// two of six in both `m365_defender` and `microsoft_defender_endpoint`.
#[test]
fn camel_keys_fold_to_pascal_with_the_os_exception() {
    let script = "Map normalized = new HashMap();\n\
         for (entry in ctx.json.entrySet()) {\n  \
         String key = entry.getKey();\n  String newKey;\n  \
         if (key.startsWith(\"os\") && key.length() > 2 \
         && Character.isUpperCase(key.charAt(2))) {\n    \
         newKey = \"OS\" + key.substring(2);\n  } else {\n    \
         newKey = key.substring(0, 1).toUpperCase() + key.substring(1);\n  }\n  \
         normalized.put(newKey, entry.getValue());\n}\nctx.json = normalized;\n";

    let mut event = Event::new(json!({"json": {
        "cveId": "CVE-2024-9143",
        "osPlatform": "Linux",
        "osVersion": "enterprise_linux_9.4",
        "deviceId": "cccccccccccccc",
        "isOnboarded": true,
    }}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("json.CveId"), Some("CVE-2024-9143"));
    assert_eq!(event.get_str("json.OSPlatform"), Some("Linux"));
    assert_eq!(
        event.get_str("json.OSVersion"),
        Some("enterprise_linux_9.4")
    );
    assert_eq!(event.get_str("json.DeviceId"), Some("cccccccccccccc"));
    assert_eq!(event.get("json.IsOnboarded"), Some(&json!(true)));

    // A key that is already PascalCase survives it, which is what lets one
    // pipeline take both spellings.
    let mut pascal = Event::new(json!({"json": {
        "CveId": "CVE-2022-49226", "OSPlatform": "Linux", "Other": 1,
    }}));
    assert!(try_known_painless(&mut pascal, script));
    assert_eq!(pascal.get_str("json.CveId"), Some("CVE-2022-49226"));
    assert_eq!(pascal.get_str("json.OSPlatform"), Some("Linux"));
    assert_eq!(pascal.get("json.Other"), Some(&json!(1)));
}

/// Verbatim from `pipelines/aws/vpcflow/default.yml`: the six TCP flags,
/// in the order the script tests them. `aws/firewall_logs` ships the same
/// ladder with double quotes.
#[test]
fn a_flag_word_decodes_into_its_names() {
    let script = "if (ctx.aws.vpcflow.tcp_flags_array == null) {\n  \
         ArrayList al = new ArrayList();\n  \
         ctx.aws.vpcflow.put(\"tcp_flags_array\", al);\n}\n\n\
         def flags = Integer.parseUnsignedInt(ctx.aws.vpcflow.tcp_flags);\n\n\
         if ((flags & 0x01) != 0) {\n  ctx.aws.vpcflow.tcp_flags_array.add('fin');\n}\n\
         if ((flags & 0x02) != 0) {\n  ctx.aws.vpcflow.tcp_flags_array.add('syn');\n}\n\
         if ((flags & 0x04) != 0) {\n  ctx.aws.vpcflow.tcp_flags_array.add('rst');\n}\n\
         if ((flags & 0x08) != 0) {\n  ctx.aws.vpcflow.tcp_flags_array.add('psh');\n}\n\
         if ((flags & 0x10) != 0) {\n  ctx.aws.vpcflow.tcp_flags_array.add('ack');\n}\n\
         if ((flags & 0x20) != 0) {\n  ctx.aws.vpcflow.tcp_flags_array.add('urg');\n}\n";

    // SYN alone.
    let mut syn = Event::new(json!({"aws": {"vpcflow": {"tcp_flags": "2"}}}));
    assert!(try_known_painless(&mut syn, script));
    assert_eq!(
        syn.get("aws.vpcflow.tcp_flags_array"),
        Some(&json!(["syn"]))
    );

    // SYN + ACK, in the script's own test order rather than the input's.
    let mut synack = Event::new(json!({"aws": {"vpcflow": {"tcp_flags": "18"}}}));
    assert!(try_known_painless(&mut synack, script));
    assert_eq!(
        synack.get("aws.vpcflow.tcp_flags_array"),
        Some(&json!(["syn", "ack"]))
    );

    // No bits set writes an empty list, which is what the script's own
    // `new ArrayList()` leaves behind.
    let mut none = Event::new(json!({"aws": {"vpcflow": {"tcp_flags": "0"}}}));
    assert!(try_known_painless(&mut none, script));
    assert_eq!(none.get("aws.vpcflow.tcp_flags_array"), Some(&json!([])));
}

/// Verbatim from `pipelines/netflow/log/default.yml`. Bit 8 down to 1
/// against `flags[8-i]`, so the top bit (0x80, CWR) is `flags[0]` and the
/// bottom bit (0x01, FIN) is `flags[7]`. The two masks are read straight
/// off `testdata/compat/netflow/log/test-netflow-log-events/expected.ndjson`.
#[test]
fn tcp_control_bits_decode_high_bit_first() {
    let script = r#"String[] flags = new String[]{\"CWR\", \"ECE\", \"URG\", \"ACK\", \"PSH\", \"RST\", \"SYN\", \"FIN\"};\nArrayList flagsSeen = new ArrayList();\n\nint tcp_flags = ctx.netflow.tcp_control_bits;\nfor (int i = 8; i > 0; --i) {\n    int value = (tcp_flags & (1 << (i-1))) & 0x0ff;\n    if (value != 0) {\n        flagsSeen.add(flags[8-i]);\n    }\n}\n\nif (flagsSeen.length > 0) {\n    ctx.netflow.tcp_flags = flagsSeen;\n}\n"#;

    // 27 = 0b00011011 = ACK|PSH|SYN|FIN.
    let mut event = Event::new(json!({"netflow": {"tcp_control_bits": 27}}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("netflow.tcp_flags"),
        Some(&json!(["ACK", "PSH", "SYN", "FIN"]))
    );

    // 56 = 0b00111000 = URG|ACK|PSH.
    let mut event = Event::new(json!({"netflow": {"tcp_control_bits": 56}}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("netflow.tcp_flags"),
        Some(&json!(["URG", "ACK", "PSH"]))
    );

    // No bit set is the script's own `if (flagsSeen.length > 0)` guard --
    // nothing is written, not an empty list.
    let mut none = Event::new(json!({"netflow": {"tcp_control_bits": 0}}));
    assert!(try_known_painless(&mut none, script));
    assert!(!none.has("netflow.tcp_flags"));
}

/// Verbatim from `pipelines/aws/waf/default.yml`: the same name/value fold
/// proofpoint writes with `put`, spelled as an indexed loop over a
/// subscripted map.
#[test]
fn an_indexed_name_value_loop_folds_into_a_map() {
    let script = "if (ctx.json.httpRequest.headers != null) {\n  \
         ctx.aws.waf.request = new HashMap();\n  \
         ctx.aws.waf.request.headers = new HashMap();\n  \
         for (def i = 0; i < ctx.json.httpRequest.headers.length; i++) {\n    \
         ctx.aws.waf.request.headers[ctx.json.httpRequest.headers[i].name] = \
         ctx.json.httpRequest.headers[i].value;\n  }\n}";

    let mut event = Event::new(json!({
        "json": { "httpRequest": { "headers": [
            {"name": "Host", "value": "localhost:1989"},
            {"name": "User-Agent", "value": "curl/7.61.1"},
            {"name": "Accept", "value": "*/*"},
        ]}}
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get_str("aws.waf.request.headers.Host"),
        Some("localhost:1989")
    );
    assert_eq!(
        event.get_str("aws.waf.request.headers.User-Agent"),
        Some("curl/7.61.1")
    );
    assert_eq!(event.get_str("aws.waf.request.headers.Accept"), Some("*/*"));
}

/// Verbatim from `pipelines/aws/s3access/default.yml`, which lowercases
/// before the split. Taking everything before `.splitOnToken(` read
/// `toLowerCase()` as a segment of the field's path.
#[test]
fn a_lowercased_tls_token_still_names_its_field() {
    let script = "def parts = ctx.aws.s3access.tls_version.toLowerCase().splitOnToken(\"v\");\n\
        if (parts.length != 2) {\n  return;\n}\n\
        ctx.tls.version = parts[1];\n\
        ctx.tls.version_protocol = parts[0]";

    let mut event = Event::new(json!({
        "aws": { "s3access": { "tls_version": "TLSv1.2" }},
        "tls": { "cipher": "ECDHE-RSA-AES128-GCM-SHA256" },
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("tls.version"), Some("1.2"));
    assert_eq!(event.get_str("tls.version_protocol"), Some("tls"));
}

/// Verbatim from `pipelines/cisco/umbrella/default.yml`: the identities
/// dance's first half, the tail of one field past another's length.
#[test]
fn prefix_tail_reads_its_three_paths_and_strips_the_comma() {
    let script = "String identities_tail = ctx.cisco.umbrella.identities.substring(ctx.cisco.umbrella.identity.length());\n\
        if (identities_tail.startsWith(',')) {\n  identities_tail = identities_tail.substring(1);\n}\n\
        if (ctx.cisco.umbrella._tmp == null) {\n  ctx.cisco.umbrella._tmp = new HashMap();\n}\n\
        ctx.cisco.umbrella._tmp.identities_tail = identities_tail;";
    let mut event = Event::new(json!({
        "cisco": { "umbrella": {
            "identity": "Last, First (f.last@example.com)",
            "identities": "Last, First (f.last@example.com),HOSTNAME1",
        }},
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("cisco.umbrella._tmp.identities_tail"),
        Some(&json!("HOSTNAME1"))
    );
}

/// The dance's second half: the scalar prepended to the split tail.
#[test]
fn prepend_to_array_rebuilds_the_list() {
    let script = "def identities = new ArrayList();\n\
        identities.add(ctx.cisco.umbrella.identity);\n\
        for (identity in ctx.cisco.umbrella._tmp.identities_tail) {\n  identities.add(identity);\n}\n\
        ctx.cisco.umbrella._tmp.identities = identities;";
    let mut event = Event::new(json!({
        "cisco": { "umbrella": {
            "identity": "Last, First (f.last@example.com)",
            "_tmp": { "identities_tail": ["HOSTNAME1", "HOSTNAME2"] },
        }},
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("cisco.umbrella._tmp.identities"),
        Some(&json!([
            "Last, First (f.last@example.com)",
            "HOSTNAME1",
            "HOSTNAME2"
        ]))
    );
}

/// The scripts below are the verbatim text the transform modules pass
/// to `painless_exec`, so a change upstream shows up here as a miss.
/// Verbatim from `pipelines/cisco_asa/default.yml`, tagged
/// `script_process_flow_duration`. `cisco_ftd` ships it too.
const FLOW_DURATION: &str = "long parse_hms(String s) {\n    long cur = 0, total = 0;\n    \
    for (char c: s.toCharArray()) {\n        if (c >= (char)'0' && c <= (char)'9') {\n    \
    cur = (cur*10) + (long)c - (char)'0';\n        } else if (c == (char)':') {\n    \
    total = (total + cur) * 60;\n            cur = 0;\n        }\n    }\n    \
    return total + cur;\n}\nif (ctx?.event == null) {\n    ctx['event'] = new HashMap();\n}\n\
    long nanos = parse_hms(ctx._temp_.duration_hms) * 1000000000L;\n\
    ctx.event['duration'] = nanos;\nif (ctx['@timestamp'] != null) {\n    \
    String end = ctx['@timestamp'];\n    ctx.event['end'] = end;\n    try {\n        \
    ctx.event['start'] = ZonedDateTime.ofInstant(\n            \
    Instant.parse(end).minusNanos(nanos),\n            ZoneOffset.UTC);\n    } \
    catch (Exception e) {\n    }\n}\n";

/// The colon form is positional: `0:01:07` is 67 seconds, and the start is
/// that far before the event's own timestamp.
#[test]
fn a_flow_duration_becomes_a_span_ending_at_the_timestamp() {
    let mut event = Event::new(json!({
        "@timestamp": "2018-10-10T12:34:56.000Z",
        "_temp_": { "duration_hms": "0:01:07" },
    }));

    assert!(try_known_painless(&mut event, FLOW_DURATION));

    assert_eq!(event.get("event.duration"), Some(&json!(67_000_000_000i64)));
    assert_eq!(
        event.get("event.end"),
        Some(&json!("2018-10-10T12:34:56.000Z"))
    );
    assert_eq!(
        event.get("event.start"),
        Some(&json!("2018-10-10T12:33:49.000Z"))
    );
}

/// Verbatim from `pipelines/cyberarkpas/audit/audit.yml`, tagged
/// `script_set_event_duration_from_the_session_duration_hh_mm_ss`. Same
/// job as cisco's, written by a different hand: an indexed loop rather
/// than `toCharArray`, `_tmp` rather than `_temp_`, and no span.
const SESSION_DURATION: &str = "long parse_hms(String s) {\n    \
    long cur = 0, total = 0;\n    for (int i = 0, n = s.length(); i < n; i++) {\n        \
    char c = s.charAt(i);\n        if (c >= (char)'0' && c <= (char)'9') {\n            \
    cur = (cur*10) + (long)(c - (char)'0');\n        } else if (c == (char)':') {\n            \
    total = (total + cur) * 60;\n            cur = 0;\n        } else {\n            \
    return 0;\n        }\n    }\n    return total + cur;\n}\n\
    long nanos = parse_hms(ctx._tmp.duration_hms) * 1000000000L;\n\
    ctx.event['duration'] = nanos;\n";

#[test]
fn a_session_duration_is_read_from_the_scripts_own_tmp_field() {
    let mut event = Event::new(json!({
        "@timestamp": "2018-10-10T12:34:56.000Z",
        "_tmp": { "duration_hms": "01:02:03" },
    }));

    assert!(try_known_painless(&mut event, SESSION_DURATION));

    assert_eq!(
        event.get("event.duration"),
        Some(&json!(3_723_000_000_000i64))
    );
    // No span: this script writes only the duration, and inventing the
    // ends from the cisco pattern would put two fields on the document
    // that Elasticsearch never wrote.
    assert_eq!(event.get("event.start"), None);
    assert_eq!(event.get("event.end"), None);
}

/// With no timestamp there is nothing to count back from, so the duration
/// is written and the span is not.
#[test]
fn a_flow_duration_without_a_timestamp_sets_only_the_duration() {
    let mut event = Event::new(json!({ "_temp_": { "duration_hms": "0:00:05" } }));

    assert!(try_known_painless(&mut event, FLOW_DURATION));

    assert_eq!(event.get("event.duration"), Some(&json!(5_000_000_000i64)));
    assert_eq!(event.get("event.start"), None);
    assert_eq!(event.get("event.end"), None);
}

/// Verbatim from `pipelines/cisco_umbrella/default.yml`, cut to two of its
/// three helpers and their rules.
const IDENTITIES: &str = "void setUser(def ctx, def x) {\n  if (ctx.user == null) {\n    \
    ctx.user = new HashMap();\n  }\n  if (ctx.user.name == null) {\n    \
    ctx.user.name = x;\n  }\n}\nvoid addNetwork(def ctx, def x) {\n  \
    if (ctx.network == null) {\n    ctx.network = new HashMap();\n  }\n  \
    if (ctx.network?.name == null) {\n    ArrayList al = new ArrayList();\n    \
    ctx.network.put(\"name\", al);\n  }\n  if (!ctx.network.name.contains(x)) {\n    \
    ctx.network.name.add(x);\n  }\n}\ndef i = 0;\n\
    for (cisco_identity_type in ctx.cisco.umbrella.identity_types) {\n  \
    if ([\"AD Users\"].contains(cisco_identity_type)) {\n    \
    setUser(ctx, ctx.cisco.umbrella.identities[i]);\n  }\n  \
    if ([\"Sites\", \"Internal Networks\", \"Networks\"].contains(cisco_identity_type)) {\n    \
    addNetwork(ctx, ctx.cisco.umbrella.identities[i]);\n  }\n  i++;\n}";

/// Each identity goes where its KIND says, position for position.
#[test]
fn parallel_identities_go_where_their_kind_says() {
    let mut event = Event::new(json!({
        "cisco": { "umbrella": {
            "identities": ["elasticuser", "Users-Internal", "Default Site"],
            "identity_types": ["AD Users", "Internal Networks", "Sites"],
        } },
    }));

    assert!(try_known_painless(&mut event, IDENTITIES));

    assert_eq!(event.get("user.name"), Some(&json!("elasticuser")));
    assert_eq!(
        event.get("network.name"),
        Some(&json!(["Users-Internal", "Default Site"]))
    );
}

/// A name already there is kept: the helper only sets what is absent.
#[test]
fn a_name_already_set_is_not_replaced() {
    let mut event = Event::new(json!({
        "user": { "name": "already-here" },
        "cisco": { "umbrella": {
            "identities": ["elasticuser"],
            "identity_types": ["AD Users"],
        } },
    }));

    assert!(try_known_painless(&mut event, IDENTITIES));

    assert_eq!(event.get("user.name"), Some(&json!("already-here")));
}

/// A kind no rule names contributes nothing.
#[test]
fn an_unnamed_identity_kind_is_ignored() {
    let mut event = Event::new(json!({
        "cisco": { "umbrella": {
            "identities": ["something"],
            "identity_types": ["Some Future Kind"],
        } },
    }));

    assert!(try_known_painless(&mut event, IDENTITIES));

    assert_eq!(event.get("user.name"), None);
    assert_eq!(event.get("network.name"), None);
}

/// Verbatim from `pipelines/cisco_ftd/default.yml`. FTD writes both
/// readings as two branches: for 430003 the timestamp is the START.
const FLOW_BOTH_WAYS: &str = "long parse_hms(String s) {\n    long cur = 0, total = 0;\n    \
    for (char c: s.toCharArray()) {\n        cur = cur;\n    }\n    return total + cur;\n} \
    if (ctx.event == null) {\n    ctx['event'] = new HashMap();\n} \
    if (ctx?._temp_.cisco?.message_id == '430003') {\n  String start = ctx['@timestamp'];\n  \
    ctx.event['start'] = start;\n  long nanos = parse_hms(ctx._temp_.duration_hms) * \
    1000000000L;\n  ctx.event['duration'] = nanos;\n  ctx.event['end'] = \
    ZonedDateTime.ofInstant(\n      Instant.parse(start).plusNanos(nanos),\n      \
    ZoneOffset.UTC);\n} else {\n  String end = ctx['@timestamp'];\n  \
    ctx.event['end'] = end;\n  long nanos = parse_hms(ctx._temp_.duration_hms) * \
    1000000000L;\n  ctx.event['duration'] = nanos;\n  ctx.event['start'] = \
    ZonedDateTime.ofInstant(\n      Instant.parse(end).minusNanos(nanos),\n      \
    ZoneOffset.UTC);\n}\n";

/// 430003 is timestamped at the start, so the end is counted FORWARD.
#[test]
fn a_start_anchored_flow_counts_the_end_forward() {
    let mut event = Event::new(json!({
        "@timestamp": "2018-10-10T12:33:49.000Z",
        "_temp_": { "cisco": { "message_id": "430003" }, "duration_hms": "0:01:07" },
    }));

    assert!(try_known_painless(&mut event, FLOW_BOTH_WAYS));

    assert_eq!(
        event.get("event.start"),
        Some(&json!("2018-10-10T12:33:49.000Z"))
    );
    assert_eq!(
        event.get("event.end"),
        Some(&json!("2018-10-10T12:34:56.000Z"))
    );
    assert_eq!(event.get("event.duration"), Some(&json!(67_000_000_000i64)));
}

/// Every other id is timestamped at the end, so the start is counted BACK.
#[test]
fn an_end_anchored_flow_counts_the_start_back() {
    let mut event = Event::new(json!({
        "@timestamp": "2018-10-10T12:34:56.000Z",
        "_temp_": { "cisco": { "message_id": "430002" }, "duration_hms": "0:01:07" },
    }));

    assert!(try_known_painless(&mut event, FLOW_BOTH_WAYS));

    assert_eq!(
        event.get("event.start"),
        Some(&json!("2018-10-10T12:33:49.000Z"))
    );
    assert_eq!(
        event.get("event.end"),
        Some(&json!("2018-10-10T12:34:56.000Z"))
    );
}

const SUM_BYTES: &str = "ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes";
const SUM_PACKETS: &str = "ctx.network.packets = ctx.source.packets + ctx.destination.packets";
const DURATION_NANOS: &str =
    "ctx.event.duration = Long.parseLong(ctx.fortinet.firewall.duration) * 1000000000";
/// Verbatim from `pipelines/fortinet/default.yml`, trimmed to four arms.
const IANA_LADDER: &str = "def iana_number = ctx.network.iana_number;\nif (iana_number == '0') \
                           {\n    ctx.network.transport = 'hopopt';\n} else if (iana_number == \
                           '1') {\n    ctx.network.transport = 'icmp';\n} else if (iana_number \
                           == '6') {\n    ctx.network.transport = 'tcp';\n} else if \
                           (iana_number == '17') {\n    ctx.network.transport = 'udp';\n}";
const APPEND_DNS: &str = "def dnsIPs = ctx.dns?.resolved_ip;\nif (dnsIPs != null) {\n  \
                          for (ip in dnsIPs) {\n    if (!ctx.related.ip.contains(ip)) \
                          {\n ctx.related.ip.add(ip);\n }\n  }\n}";
/// Verbatim from `pipelines/fortinet/event.yml`. fortinet's VPN logs are
/// back to front by ECS's reckoning: `remip` is the client and `locip` the
/// firewall, so the pipeline renames them the obvious way and then swaps.
const VPN_SWAP: &str = "def tmp = ctx.source;\nctx.source = ctx.destination;\n\
                        if (ctx.source == null) { ctx.source = [:]; }\n\
                        if ( tmp?.user != null ) {\n    ctx.source.user = tmp.user;\n    \
                        tmp.remove(\"user\");\n}\nctx.destination = tmp;";

/// Verbatim from `pipelines/o365/audit.yml`, which collects a mail rule's
/// forwarding addresses out of three optional parameters.
const SPLIT_TRIM_ADD: &str = "void splitTrimAdd(Set acc, String str) {\n    \
    if (str != null && str != '') {\n        String[] parts = str.splitOnToken(';');\n        \
    for (int i = 0; i < parts.length; i++) {\n            acc.add(parts[i].trim());\n        \
    }\n    }\n}\ndef addressSet = new HashSet(ctx.email?.to?.address ?: []);\n\
    splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.ForwardAsAttachmentTo); \
    splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.ForwardTo); \
    splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.RedirectTo);\n\
    if (!addressSet.isEmpty()) {\n  ctx.email = ctx.email ?: [:];\n  \
    ctx.email.to = ctx.email.to ?: [:];\n  ctx.email.to.address = addressSet.asList();\n}\n";

/// Verbatim from `pipelines/fortinet/utm.yml`, which splits `tlsver` into
/// the two ECS `tls` fields at the version's first digit.
const TLS_VERSION: &str = "def pat = /\\d+/; def tlsver = \
    ctx.fortinet.firewall.tlsver.toLowerCase(); def matcher = pat.matcher(tlsver); \
    if (!matcher.find()) {\n    return;\n} if (ctx.tls == null) {\n    \
    ctx.tls = new HashMap();\n} ctx.tls.version_protocol = tlsver.substring(0, \
    matcher.start()); ctx.tls.version = tlsver.substring(matcher.start(), \
    tlsver.length()); if (!ctx.tls.version.contains(\".\")) {\n    \
    ctx.tls.version += \".0\";\n}";

/// Verbatim from `pipelines/o365/default.yml`, which builds `message` for
/// a DLP-Exchange alert out of three fields that may each be absent.
const DLP_MESSAGE: &str = "def operation = ctx.event?.action ?: '';\n\
    def user = ctx.user?.id ?: '';\n\
    def subject = ctx.o365audit?.ExchangeMetaData?.Subject ?: ctx.email?.subject ?: '';\n\
    if (operation.isEmpty() && user.isEmpty() && subject.isEmpty()) {\n  \
    ctx.message = \"Office365 Alert\";\n} else {\n  \
    ctx.message = \"Office365 Alert: \" + operation + \" detected in email sent by \" + \
    user + \" with subject '\" + subject + \"'\";\n}";

/// Verbatim from `pipelines/crowdstrike/firewall_match.yml`. Every rename
/// of `LocalAddress` and `RemoteAddress` after it is gated on the result.
const DIRECTION: &str = "def result = [];\n\
    if (ctx.crowdstrike.event.ConnectionDirection == \"0\") {\n  result.add('egress');\n}\n\
    else if (ctx.crowdstrike.event.ConnectionDirection == \"1\") {\n  result.add('ingress');\n}\n\
    else if (ctx.crowdstrike.event.ConnectionDirection == \"3\") {\n  \
    result.add('egress');\n  result.add('ingress');\n}\n\
    if (result.size() > 0) {\n  ctx.network = ctx.network ?: [:];\n}\n\
    if (result.size() == 1) {\n  ctx.network.direction = result[0];\n}\n\
    else if (result.size() > 1) {\n  ctx.network.direction = result;\n}";

/// Verbatim from `pipelines/fortinet/traffic.yml`. The directional matcher
/// only knows source-plus-destination-into-network, and this is three
/// different names.
#[test]
fn a_sum_reads_all_three_names_out_of_the_script() {
    let script = "ctx.fortinet.firewall.deltabytes = ctx.fortinet.firewall.rcvddelta \
                  + ctx.fortinet.firewall.sentdelta";
    let mut event = Event::new(json!({
        "fortinet": { "firewall": { "rcvddelta": 1000, "sentdelta": 304 } },
    }));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("fortinet.firewall.deltabytes"),
        Some(&json!(1304))
    );
}

/// Elastic gates the script on both sides being a Number and would throw
/// otherwise, so an absent side writes nothing.
#[test]
fn a_sum_with_a_side_missing_writes_nothing() {
    let script = "ctx.a.total = ctx.a.left + ctx.a.right";
    let mut event = Event::new(json!({ "a": { "left": 5 } }));

    assert!(try_known_painless(&mut event, script));
    assert!(!event.has("a.total"));
}

/// The fixture line fortinet 7.4 logs: `tlsver="tls1.3"`.
#[test]
fn a_version_splits_at_its_first_digit() {
    let mut event = Event::new(json!({
        "fortinet": { "firewall": { "tlsver": "TLS1.3" } },
    }));

    assert!(try_known_painless(&mut event, TLS_VERSION));
    assert_eq!(event.get_str("tls.version_protocol"), Some("tls"));
    assert_eq!(event.get_str("tls.version"), Some("1.3"));
}

/// The o365 fixture's own line: three addresses in one semicolon-delimited
/// `ForwardTo`.
#[test]
fn split_trim_collect_gathers_every_forwarding_address() {
    let mut event = Event::new(json!({
        "o365audit": { "Parameters": {
            "ForwardTo": "external1@example.com;external2@example.com;external3@example.com",
            "RedirectTo": " spaced@example.com ",
        } },
    }));

    assert!(try_known_painless(&mut event, SPLIT_TRIM_ADD));
    assert_eq!(
        event.get("email.to.address"),
        Some(&json!([
            "external1@example.com",
            "external2@example.com",
            "external3@example.com",
            "spaced@example.com",
        ]))
    );
}

/// The set is SEEDED from what the target already holds, and a duplicate
/// coming in over the top of it is dropped.
#[test]
fn split_trim_collect_seeds_from_the_target_and_dedups() {
    let mut event = Event::new(json!({
        "email": { "to": { "address": ["already@example.com"] } },
        "o365audit": { "Parameters": { "ForwardTo": "already@example.com;new@example.com" } },
    }));

    assert!(try_known_painless(&mut event, SPLIT_TRIM_ADD));
    assert_eq!(
        event.get("email.to.address"),
        Some(&json!(["already@example.com", "new@example.com"]))
    );
}

/// `if (!addressSet.isEmpty())` -- no parameters, no field.
#[test]
fn split_trim_collect_writes_nothing_when_it_gathers_nothing() {
    let mut event = Event::new(json!({ "o365audit": { "Parameters": {} } }));

    assert!(try_known_painless(&mut event, SPLIT_TRIM_ADD));
    assert!(!event.has("email.to.address"));
}

/// A version with no dot gets `.0`, which is the script's last three lines
/// and the reason `tls1` and `tls1.0` end up the same.
#[test]
fn a_version_without_a_dot_gains_one() {
    let mut event = Event::new(json!({
        "fortinet": { "firewall": { "tlsver": "tls1" } },
    }));

    assert!(try_known_painless(&mut event, TLS_VERSION));
    assert_eq!(event.get_str("tls.version"), Some("1.0"));
}

/// `if (!matcher.find()) { return; }` -- no digit, so nothing is written
/// and the script is still counted as run.
#[test]
fn a_version_with_no_digit_writes_nothing() {
    let mut event = Event::new(json!({
        "fortinet": { "firewall": { "tlsver": "unknown" } },
    }));

    assert!(try_known_painless(&mut event, TLS_VERSION));
    assert!(!event.has("tls.version"));
    assert!(!event.has("tls.version_protocol"));
}

/// One match writes a scalar.
#[test]
fn collecting_ladder_writes_a_single_match_as_a_string() {
    let mut event = Event::new(json!({
        "crowdstrike": { "event": { "ConnectionDirection": "1" } },
    }));

    assert!(try_known_painless(&mut event, DIRECTION));
    assert_eq!(event.get_str("network.direction"), Some("ingress"));
}

/// Several matches write an array, which is the half a plain ladder cannot
/// express.
#[test]
fn collecting_ladder_writes_several_matches_as_an_array() {
    let mut event = Event::new(json!({
        "crowdstrike": { "event": { "ConnectionDirection": "3" } },
    }));

    assert!(try_known_painless(&mut event, DIRECTION));
    assert_eq!(
        event.get("network.direction"),
        Some(&json!(["egress", "ingress"]))
    );
}

/// The target is the field the accumulator is assigned to, not the parent
/// the size test creates -- reading the size test named `network`.
#[test]
fn collecting_ladder_writes_the_field_not_its_parent() {
    let mut event = Event::new(json!({
        "crowdstrike": { "event": { "ConnectionDirection": "0" } },
    }));

    assert!(try_known_painless(&mut event, DIRECTION));
    assert_eq!(event.get_str("network.direction"), Some("egress"));
    assert!(event.get("network").is_some_and(Value::is_object));
}

/// A value no arm matches leaves the field unwritten, which is what an
/// empty accumulator does.
#[test]
fn collecting_ladder_writes_nothing_when_no_arm_matches() {
    let mut event = Event::new(json!({
        "crowdstrike": { "event": { "ConnectionDirection": "9" } },
    }));

    assert!(try_known_painless(&mut event, DIRECTION));
    assert!(!event.has("network.direction"));
}

#[test]
fn concat_builds_the_message_from_the_fields_it_names() {
    let mut event = Event::new(json!({
        "event": { "action": "DlpRuleMatch" },
        "user": { "id": "DlpAgent" },
    }));

    assert!(try_known_painless(&mut event, DLP_MESSAGE));
    assert_eq!(
        event.get_str("message"),
        Some("Office365 Alert: DlpRuleMatch detected in email sent by DlpAgent with subject ''")
    );
}

/// The `?:` chain takes the first field that is present AND non-empty, so
/// an empty subject falls through to the next alternative.
#[test]
fn concat_falls_through_an_empty_alternative() {
    let mut event = Event::new(json!({
        "event": { "action": "DlpRuleMatch" },
        "o365audit": { "ExchangeMetaData": { "Subject": "" } },
        "email": { "subject": "Q3 numbers" },
    }));

    assert!(try_known_painless(&mut event, DLP_MESSAGE));
    assert_eq!(
        event.get_str("message"),
        Some("Office365 Alert: DlpRuleMatch detected in email sent by  with subject 'Q3 numbers'")
    );
}

/// Every field absent takes the other branch, which is a bare literal.
#[test]
fn concat_takes_the_literal_when_every_field_is_empty() {
    let mut event = Event::new(json!({ "event": { "code": "ComplianceDLPExchange" } }));

    assert!(try_known_painless(&mut event, DLP_MESSAGE));
    assert_eq!(event.get_str("message"), Some("Office365 Alert"));
}

#[test]
fn vpn_swap_exchanges_source_and_destination() {
    let mut event = Event::new(json!({
        "source": { "ip": "10.0.0.1", "port": 500 },
        "destination": { "ip": "203.0.113.7", "port": 500 },
    }));

    assert!(try_known_painless(&mut event, VPN_SWAP));
    assert_eq!(event.get_str("source.ip"), Some("203.0.113.7"));
    assert_eq!(event.get_str("destination.ip"), Some("10.0.0.1"));
}

/// `user` describes the person, not the address, so it stays with the
/// source rather than riding the swap across.
#[test]
fn vpn_swap_keeps_the_user_on_the_source() {
    let mut event = Event::new(json!({
        "source": { "ip": "10.0.0.1", "user": { "name": "derek" } },
        "destination": { "ip": "203.0.113.7" },
    }));

    assert!(try_known_painless(&mut event, VPN_SWAP));
    assert_eq!(event.get_str("source.ip"), Some("203.0.113.7"));
    assert_eq!(event.get_str("source.user.name"), Some("derek"));
    assert!(!event.has("destination.user"));
    assert_eq!(event.get_str("destination.ip"), Some("10.0.0.1"));
}

/// A VPN event carrying only `remip` leaves one side with nothing, and the
/// captured Elasticsearch output has no `destination` key at all -- not a
/// null one.
#[test]
fn vpn_swap_removes_a_side_left_with_nothing() {
    let mut event = Event::new(json!({ "destination": { "ip": "203.0.113.7" } }));

    assert!(try_known_painless(&mut event, VPN_SWAP));
    assert_eq!(event.get_str("source.ip"), Some("203.0.113.7"));
    assert!(!event.has("destination"), "destination survived as null");
}

/// Verbatim from `pipelines/crowdstrike/default.yml`.
const APPEND_TAGS: &str = "if (ctx.crowdstrike.event.Tags instanceof List) {\n    for (tag in \
     ctx.crowdstrike.event.Tags) {\n        if (tag instanceof Map) {\n          \
     ctx.tags.add(tag[\"Key\"] + \":\" + tag[\"ValueString\"]);\n        }\n    }\n} else if \
     (ctx.crowdstrike.event.Tags instanceof String) {\n    def values = \
     ctx.crowdstrike.event.Tags.splitOnToken(',');\n    for (value in values) {\n        \
     ctx.tags.add(value.trim());\n    }\n}";

#[test]
fn splits_a_delimited_string_onto_the_end_of_the_array() {
    let mut event = Event::new(json!({
        "crowdstrike": { "event": { "Tags": "SensorGroupingTags/TEACHER, FalconGroupingTags/X" }},
        "tags": ["preserve_original_event"],
    }));

    assert!(try_known_painless(&mut event, APPEND_TAGS));

    assert_eq!(
        event.get("tags"),
        Some(&json!([
            "preserve_original_event",
            "SensorGroupingTags/TEACHER",
            "FalconGroupingTags/X"
        ]))
    );
}

/// The same script's other branch: the field arrives as maps, not a string.
#[test]
fn joins_each_map_pair_onto_the_end_of_the_array() {
    let mut event = Event::new(json!({
        "crowdstrike": { "event": { "Tags": [
            { "Key": "env", "ValueString": "prod" },
            { "Key": "team", "ValueString": "sec" },
        ]}},
        "tags": ["preserve_original_event"],
    }));

    assert!(try_known_painless(&mut event, APPEND_TAGS));

    assert_eq!(
        event.get("tags"),
        Some(&json!(["preserve_original_event", "env:prod", "team:sec"]))
    );
}

#[test]
fn an_absent_source_leaves_the_array_alone() {
    let mut event = Event::new(json!({ "tags": ["preserve_original_event"] }));
    assert!(try_known_painless(&mut event, APPEND_TAGS));
    assert_eq!(event.get("tags"), Some(&json!(["preserve_original_event"])));
}

/// Shortened from `pipelines/fortinet/default.yml` -- the parts that
/// identify the pattern, not the whole 30-line definition.
const SPLIT_UNQUOTED: &str = "def splitUnquoted(String input, String sep) {\n  def tokens = \
                              [];\n}\ndef arr = splitUnquoted(ctx.syslog5424_sd, \" \");\n\
                              Map map = new HashMap();\nfor (def i = 0; i < arr?.length; i++) \
                              {\n  def kv = splitUnquoted(arr[i], \"=\");\n}\n\
                              ctx.fortinet.firewall = map;\n";

#[test]
fn a_quoted_value_keeps_its_spaces() {
    let mut event = Event::new(json!({
        "syslog5424_sd": "type=\"utm\" msg=\"URL belongs to a denied category\" policyid=100602",
    }));

    assert!(try_known_painless(&mut event, SPLIT_UNQUOTED));

    assert_eq!(event.get_str("fortinet.firewall.type"), Some("utm"));
    assert_eq!(
        event.get_str("fortinet.firewall.msg"),
        Some("URL belongs to a denied category")
    );
    assert_eq!(event.get_str("fortinet.firewall.policyid"), Some("100602"));
}

/// The vendor's `kv.length == 2` guard: a fragment with no `=` is skipped.
#[test]
fn a_fragment_without_the_pair_separator_is_skipped() {
    let mut event = Event::new(json!({ "syslog5424_sd": "bare a=1" }));
    assert!(try_known_painless(&mut event, SPLIT_UNQUOTED));

    let map = event.get_object("fortinet.firewall").unwrap();
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("a").and_then(Value::as_str), Some("1"));
}

#[test]
fn sums_bytes_and_packets_across_directions() {
    let mut event = Event::new(json!({
        "source": { "bytes": 100, "packets": 3 },
        "destination": { "bytes": 250, "packets": 4 },
    }));

    assert!(try_known_painless(&mut event, SUM_BYTES));
    assert!(try_known_painless(&mut event, SUM_PACKETS));

    assert_eq!(event.get_i64("network.bytes"), Some(350));
    assert_eq!(event.get_i64("network.packets"), Some(7));
}

/// Elastic's script throws when a side is missing; skipping is what the
/// surrounding pipeline already relies on.
#[test]
fn a_missing_direction_leaves_the_total_unset() {
    let mut event = Event::new(json!({ "source": { "bytes": 100 } }));
    assert!(try_known_painless(&mut event, SUM_BYTES));
    assert!(!event.has("network.bytes"));
}

/// Verbatim from `pipelines/zscaler_zia/dns/default.yml`, folded onto one
/// line the way YAML's `>-` delivers it. The target comes from the
/// assignment that owns the multiply, not the first `=` -- that one is the
/// `==` of the guard, and reading it wrote nothing at all.
#[test]
fn a_scaled_field_lands_on_its_own_target() {
    let script = "if (ctx.event == null) { ctx.put('event', new HashMap()); } \
         ctx.event.duration = ctx.zscaler_zia.dns.duration.milliseconds * 1000000;";

    let mut event = Event::new(json!({
        "zscaler_zia": {"dns": {"duration": {"milliseconds": 1000}}}
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_i64("event.duration"), Some(1_000_000_000));
    assert_eq!(
        event.get_i64("zscaler_zia.dns.duration.milliseconds"),
        Some(1000),
        "the source is read, not consumed"
    );

    // An absent source writes nothing rather than a zero.
    let mut empty = Event::new(json!({}));
    assert!(try_known_painless(&mut empty, script));
    assert!(!empty.has("event.duration"));
}

/// Verbatim from `pipelines/m365_defender/event/pipeline_alert.yml`.
#[test]
fn an_attack_technique_splits_into_a_name_and_an_id() {
    let script = "def subtechnique_name = new ArrayList();\n\
         def subtechnique_id = new ArrayList();\n\
         if (!(ctx.threat instanceof HashMap)) {\n  ctx.threat = new HashMap();\n}\n\
         for (item in ctx.m365_defender.event.attack_techniques) {\n\
         subtechnique_name.add(item.substring(0,item.lastIndexOf(' ')));\n\
         subtechnique_id.add(item.substring(item.indexOf('(')+1,item.indexOf(')')));\n}\n\
         ctx.threat.technique.subtechnique.id = subtechnique_id;\n\
         ctx.threat.technique.subtechnique.name = subtechnique_name;\n";

    let mut event = Event::new(json!({ "m365_defender": { "event": {
        "attack_techniques": ["Valid Accounts (T1078)", "Cloud Accounts (T1078.004)"]
    }}}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("threat.technique.subtechnique.name"),
        Some(&json!(["Valid Accounts", "Cloud Accounts"])),
        "the loop's own order -- the pipeline sorts both lists afterwards"
    );
    assert_eq!(
        event.get("threat.technique.subtechnique.id"),
        Some(&json!(["T1078", "T1078.004"]))
    );

    // An item with no bracket throws in Painless, so nothing is written.
    let mut ragged = Event::new(json!({ "m365_defender": { "event": {
        "attack_techniques": ["Valid Accounts (T1078)", "no id here"]
    }}}));
    assert!(try_known_painless(&mut ragged, script));
    assert!(!ragged.has("threat.technique.subtechnique.id"));
}

/// Verbatim from
/// `pipelines/microsoft_defender_endpoint/machine_action/default.yml`.
#[test]
fn a_hash_lands_on_the_field_its_type_names() {
    let script = "ctx.file = ctx.file ?: [:]; ctx.file.hash = ctx.file.hash ?: [:]; \
         String fileType = ctx.mde.related_file_info.file_identifier_type.toLowerCase(); \
         String fileHash = ctx.mde.related_file_info.file_identifier; \
         if (fileType.contains('sha1')) {\n  ctx.file.hash.sha1 = fileHash;\n\
         } else if (fileType.contains('md5')) {\n  ctx.file.hash.md5 = fileHash;\n\
         } else if (fileType.contains('sha256')) {\n  ctx.file.hash.sha256 = fileHash;\n}\n";

    // `Sha1` is mixed case on the wire and the script lower-cases it.
    let mut event = Event::new(json!({ "mde": { "related_file_info": {
        "file_identifier": "aaf4c61d", "file_identifier_type": "Sha1"
    }}}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("file.hash.sha1"), Some("aaf4c61d"));
    assert!(!event.has("file.hash.md5"));

    // A type no arm names writes nothing.
    let mut unknown = Event::new(json!({ "mde": { "related_file_info": {
        "file_identifier": "abc", "file_identifier_type": "Crc32"
    }}}));
    assert!(try_known_painless(&mut unknown, script));
    assert!(!unknown.has("file.hash"));
}

/// Verbatim from `pipelines/entityanalytics_entra_id/entity/user.yml`.
#[test]
fn direct_reports_become_one_column_per_member() {
    let script = "def ids = new ArrayList();\ndef names = new ArrayList();\n\
         def emails = new ArrayList();\n\
         for (def report : ctx.entityanalytics_entra_id.user.direct_reports) {\n\
         if (report == null) { continue; }\n  if (report.id != null) { ids.add(report.id); }\n\
         if (report.user_principal_name != null) { names.add(report.user_principal_name); }\n\
         if (report.mail != null) { emails.add(report.mail); }\n}\n\
         def userObj = new HashMap();\nif (!ids.isEmpty()) { userObj.put(\"id\", ids); }\n\
         if (!names.isEmpty()) { userObj.put(\"name\", names); }\n\
         if (!emails.isEmpty()) { userObj.put(\"email\", emails); }\n\
         if (!userObj.isEmpty()) {\n  ctx.user = ctx.user ?: new HashMap();\n\
         ctx.user.entity = ctx.user.entity ?: new HashMap();\n\
         ctx.user.entity.relationships = ctx.user.entity.relationships ?: new HashMap();\n\
         ctx.user.entity.relationships.put(\"supervises\", [\"user\": userObj]);\n}\n";

    let mut event = Event::new(json!({ "entityanalytics_entra_id": { "user": {
        "direct_reports": [
            {"id": "ee55", "mail": "one@example.com", "user_principal_name": "one@example.com"},
            {"id": "ff66", "mail": "two@example.com", "user_principal_name": "two@example.com"}
        ]
    }}}));
    assert!(try_known_painless(&mut event, script));

    assert_eq!(
        event.get("user.entity.relationships.supervises.user.id"),
        Some(&json!(["ee55", "ff66"]))
    );
    assert_eq!(
        event.get("user.entity.relationships.supervises.user.email"),
        Some(&json!(["one@example.com", "two@example.com"]))
    );

    // A member no report carries leaves its column out entirely.
    let mut sparse = Event::new(json!({ "entityanalytics_entra_id": { "user": {
        "direct_reports": [{"id": "ee55"}]
    }}}));
    assert!(try_known_painless(&mut sparse, script));
    assert_eq!(
        event.get("user.entity.relationships.supervises.user.id"),
        Some(&json!(["ee55", "ff66"]))
    );
    assert!(!sparse.has("user.entity.relationships.supervises.user.email"));
}

/// Verbatim from `pipelines/windows/forwarded/security_standard.yml`.
#[test]
fn a_file_extension_is_the_name_after_its_last_dot() {
    let script = "def extIdx = ctx.file.name.lastIndexOf(\".\");\n\
         if (extIdx > -1) {\n    ctx.file.extension = ctx.file.name.substring(extIdx+1);\n}";

    let mut event = Event::new(json!({ "file": { "name": "summary.docx" } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("file.extension"), Some("docx"));

    // A name with no dot writes nothing.
    let mut bare = Event::new(json!({ "file": { "name": "summary" } }));
    assert!(try_known_painless(&mut bare, script));
    assert!(!bare.has("file.extension"));
}

/// Verbatim from `pipelines/m365_defender/event/pipeline_device.yml`.
#[test]
fn dns_header_flags_are_the_keys_the_map_marks_true() {
    let script = "def af = ctx.m365_defender.event.additional_fields;\n\
         List ecs_flags = [\"AA\", \"TC\", \"RD\", \"RA\", \"AD\", \"CD\", \"DO\"];\n\
         List flags = [];\n\
         if (af instanceof Map) {\n    for (def flag: ecs_flags) {\n\
         if (af[flag] != null && af[flag] == \"true\") {\n            flags.add(flag);\n\
         }\n    }\n}\n\
         if (!ctx.m365_defender.event.containsKey('dns')) {\n\
         ctx.m365_defender.event.dns = new HashMap();\n}\n\
         ctx.m365_defender.event.dns.header_flags = flags;\n";

    let mut event = Event::new(json!({ "m365_defender": { "event": { "additional_fields": {
        "AA": "false", "TC": "false", "RD": "true", "RA": "true"
    }}}}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("m365_defender.event.dns.header_flags"),
        Some(&json!(["RD", "RA"])),
        "the ECS order of the literal list, not the map's"
    );
}

/// Verbatim from `pipelines/coredns/log/default.yml`. The loop walks
/// forward while removing from the list it indexes, which skips whatever
/// sits right after a removal, but `QR` is a one-bit DNS flag that is
/// never present twice, so the skip never bites. `testdata/compat/coredns`
/// is real Elasticsearch 9.2.2 output and shows `QR` removed on every
/// event that reaches this script, which rules out `.length` throwing on
/// a List here.
#[test]
fn qr_is_dropped_from_header_flags() {
    let script = r"for (int i=0; i<ctx.dns.header_flags.length; i++) {\n  if (ctx.dns.header_flags[i] == 'QR') {\n    ctx.dns.header_flags.remove(i);\n  }\n}\n";

    // `test-coredns/expected.ndjson` line 1: "qr,aa,rd" uppercased and
    // split becomes ["QR","AA","RD"] before this script runs.
    let mut event = Event::new(json!({"dns": {"header_flags": ["QR", "AA", "RD"]}}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("dns.header_flags"), Some(&json!(["AA", "RD"])));

    // `test-coredns-json/expected.ndjson` line 1: "qr,rd,ra".
    let mut event = Event::new(json!({"dns": {"header_flags": ["QR", "RD", "RA"]}}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("dns.header_flags"), Some(&json!(["RD", "RA"])));

    // No QR present leaves the list untouched.
    let mut none = Event::new(json!({"dns": {"header_flags": ["RD", "RA"]}}));
    assert!(try_known_painless(&mut none, script));
    assert_eq!(none.get("dns.header_flags"), Some(&json!(["RD", "RA"])));
}

/// Verbatim from `pipelines/m365_defender/event/pipeline_device.yml`.
const ZIP_ANSWERS: &str = "def answers = ctx.m365_defender.event.dns.answers; \
     def ttls = ctx.m365_defender.event.dns.ttls; \
     if (answers.isEmpty() || ttls.isEmpty()) {\n  return;\n} \
     else if (answers.length != ttls.length) {\n  if (ctx.error == null) {\n\
     ctx.error = new HashMap();\n  }\n  if (ctx.error.message == null) {\n\
     ctx.error.message = new ArrayList();\n  }\n\
     ctx.error.message.add('DNS answers and TTLs have a different length');\n} \
     def lst = new ArrayList(); for (def i = 0; i < answers.length; i++) {\n\
     lst.add([\n    \"data\": answers[i],\n    \"ttl\": (long)ttls[i]\n  ])\n} \
     if (ctx.dns == null) {\n  ctx.dns = new HashMap();\n} ctx.dns.answers = lst;";

#[test]
fn dns_answers_zip_with_their_ttls() {
    let mut event = Event::new(json!({ "m365_defender": { "event": { "dns": {
        "answers": ["89.160.20.112", "google.com"], "ttls": [5.0, 5.0]
    }}}}));
    assert!(try_known_painless(&mut event, ZIP_ANSWERS));
    assert_eq!(
        event.get("dns.answers"),
        Some(&json!([
            {"data": "89.160.20.112", "ttl": 5},
            {"data": "google.com", "ttl": 5}
        ])),
        "the TTL carries the script's own cast to long"
    );

    // Lists of different lengths are the script's stated error, and the one
    // that runs short throws before anything is written.
    let mut ragged = Event::new(json!({ "m365_defender": { "event": { "dns": {
        "answers": ["a", "b"], "ttls": [5.0]
    }}}}));
    assert!(try_known_painless(&mut ragged, ZIP_ANSWERS));
    assert!(!ragged.has("dns.answers"));
    assert_eq!(
        ragged.get("error.message"),
        Some(&json!(["DNS answers and TTLs have a different length"])),
        "the vendor's error.message is a list it appends to"
    );
}

/// The pattern of `pipelines/m365_defender/incident/default.yml`'s
/// `set_process_name_from_command_line`, which four pipelines share.
const PROCESS_NAME: &str = "ctx.process = ctx.process ?: [:];\n\
     ctx.process.name = ctx.process.name ?: [];\n\
     def currentNames = new HashSet();\n\
     if (ctx.process.command_line != null) { ... }\n\
     ctx.process.name = new ArrayList(currentNames);\n";

#[test]
fn a_process_name_comes_off_its_command_line() {
    // One name survives, so the field is a SCALAR.
    let mut event = Event::new(json!({ "process": { "command_line": ["\"MsSense.exe\""] } }));
    assert!(try_known_painless(&mut event, PROCESS_NAME));
    assert_eq!(event.get_str("process.name"), Some("MsSense.exe"));

    // A posix path keeps its last segment; a windows one is left whole,
    // because the vendor splits on `/` alone.
    let mut paths = Event::new(json!({ "process": { "command_line": [
        "/usr/bin/curl -s http://x", "C:\\Windows\\System32\\cmd.exe"
    ]}}));
    assert!(try_known_painless(&mut paths, PROCESS_NAME));
    assert_eq!(
        paths.get("process.name"),
        Some(&json!(["curl", "C:\\Windows\\System32\\cmd.exe"]))
    );

    // Nothing to gather writes nothing, not an empty list.
    let mut empty = Event::new(json!({ "process": { "pid": 4 } }));
    assert!(try_known_painless(&mut empty, PROCESS_NAME));
    assert!(!empty.has("process.name"));
}

/// Verbatim from `pipelines/microsoft_defender_endpoint/log/default.yml`.
#[test]
fn a_duration_is_the_span_between_two_parsed_instants() {
    let script = "Instant eventstart = ZonedDateTime.parse(ctx.event.start).toInstant(); \
         Instant eventend = ZonedDateTime.parse(ctx.event.end).toInstant(); \
         ctx.event['duration'] = ChronoUnit.NANOS.between(eventstart, eventend);\n";

    let mut event = Event::new(json!({ "event": {
        "start": "2020-07-06T05:23:56.7191052Z",
        "end": "2020-07-06T06:04:39.4188046Z"
    }}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_i64("event.duration"), Some(2_442_699_699_400));

    // An end Painless cannot parse throws, and the vendor writes nothing.
    let mut unparseable = Event::new(json!({ "event": {
        "start": "2020-07-06T05:23:56.7191052Z", "end": "not a time"
    }}));
    assert!(try_known_painless(&mut unparseable, script));
    assert!(!unparseable.has("event.duration"));
}

/// Verbatim from `crowdstrike/data_stream/alert`, which binds the span to a
/// local and copies it out under a sign guard.
///
/// The target was read as `event.end` -- the last `ctx.` path before the
/// assignment is the `end` declaration, not the field being written -- so
/// the span overwrote the timestamp a `set` processor had just copied there
/// and `event.duration` was never written at all.
#[test]
fn a_span_bound_to_a_local_lands_on_the_field_it_is_copied_to() {
    let script = "def start = ZonedDateTime.parse(ctx.event.start);\n\
         def end = ZonedDateTime.parse(ctx.event.end);\n\
         def duration = ChronoUnit.NANOS.between(start, end);\n\
         if (duration >= 0) {\n  ctx.event.duration = duration;\n}";

    let mut event = Event::new(json!({ "event": {
        "start": "2026-05-11T05:11:47.000Z",
        "end": "2026-05-11T05:13:20.000Z"
    }}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_i64("event.duration"), Some(93_000_000_000));
    assert_eq!(event.get_str("event.end"), Some("2026-05-11T05:13:20.000Z"));

    // The script's own guard: a reversed span writes nothing.
    let mut reversed = Event::new(json!({ "event": {
        "start": "2026-05-11T05:13:20.000Z",
        "end": "2026-05-11T05:11:47.000Z"
    }}));
    assert!(try_known_painless(&mut reversed, script));
    assert!(!reversed.has("event.duration"));
    assert_eq!(
        reversed.get_str("event.end"),
        Some("2026-05-11T05:11:47.000Z")
    );
}

/// Verbatim from `crowdstrike/data_stream/alert`, whose two severity
/// scripts chain: a number becomes a name, and the name becomes a score.
#[test]
fn a_severity_number_becomes_a_name_and_the_name_a_score() {
    let to_name = "long severity = ctx.crowdstrike.alert.severity;\n\
         if (0 <= severity && severity < 20) {\n  \
         ctx.crowdstrike.alert.severity_name = \"info\";\n\
         } else if (20 <= severity && severity < 40) {\n  \
         ctx.crowdstrike.alert.severity_name = \"low\";\n\
         } else if (40 <= severity && severity < 60) {\n  \
         ctx.crowdstrike.alert.severity_name = \"medium\";\n\
         } else if (60 <= severity && severity < 80) {\n  \
         ctx.crowdstrike.alert.severity_name = \"high\";\n\
         } else if (80 <= severity && severity <= 100) {\n  \
         ctx.crowdstrike.alert.severity_name = \"critical\";\n}";

    for (severity, name) in [
        (0, "info"),
        (25, "low"),
        (55, "medium"),
        (70, "high"),
        (100, "critical"),
    ] {
        let mut event = Event::new(json!({"crowdstrike": {"alert": {"severity": severity}}}));
        assert!(
            try_known_painless(&mut event, to_name),
            "{severity} unmatched"
        );
        assert_eq!(
            event.get_str("crowdstrike.alert.severity_name"),
            Some(name),
            "severity {severity}"
        );
    }

    // Outside every band the script falls through and writes nothing.
    let mut outside = Event::new(json!({"crowdstrike": {"alert": {"severity": 101}}}));
    assert!(try_known_painless(&mut outside, to_name));
    assert!(!outside.has("crowdstrike.alert.severity_name"));

    let to_score = "ctx.event = ctx.event ?: [:];\n\
         String risk_score_value = ctx.crowdstrike.alert.severity_name;\n\
         if (risk_score_value.equalsIgnoreCase(\"low\") || \
         risk_score_value.equalsIgnoreCase(\"info\") || \
         risk_score_value.equalsIgnoreCase(\"informational\")) {\n  \
         ctx.event.severity = 21;\n\
         } else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  \
         ctx.event.severity = 47;\n\
         } else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  \
         ctx.event.severity = 73;\n\
         } else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  \
         ctx.event.severity = 99;\n}";

    for (name, score) in [
        ("info", 21),
        ("low", 21),
        ("medium", 47),
        ("high", 73),
        ("critical", 99),
    ] {
        let mut event = Event::new(json!({"crowdstrike": {"alert": {"severity_name": name}}}));
        assert!(try_known_painless(&mut event, to_score), "{name} unmatched");
        assert_eq!(event.get_i64("event.severity"), Some(score), "name {name}");
    }
}

/// Verbatim from `crowdstrike/data_stream/alert`: a map scanned for one
/// key, falling back to a literal when it is absent.
#[test]
fn a_named_map_entry_is_lifted_out_with_a_default() {
    let script = "if (ctx.crowdstrike == null) {\n  ctx.crowdstrike = [:];\n}\n\
         if (ctx.crowdstrike.alert == null) {\n  ctx.crowdstrike.alert = [:];\n}\n\
         for (def d: ctx.crowdstrike.alert.pattern_disposition_details.entrySet()) {\n  \
         if (d.getKey() == 'quarantine_file') {\n    \
         ctx.crowdstrike.alert.is_synthetic_quarantine_disposition = d.getValue();\n    \
         return;\n  }\n}\n\
         ctx.crowdstrike.alert.is_synthetic_quarantine_disposition = false;\n";

    for found in [true, false] {
        let mut event = Event::new(json!({"crowdstrike": {"alert": {
            "pattern_disposition_details": {"quarantine_file": found, "other": true}
        }}}));
        assert!(try_known_painless(&mut event, script), "{found} unmatched");
        assert_eq!(
            event.get("crowdstrike.alert.is_synthetic_quarantine_disposition"),
            Some(&json!(found))
        );
    }

    // The key absent takes the literal the script falls through to.
    let mut absent = Event::new(json!({"crowdstrike": {"alert": {
        "pattern_disposition_details": {"indicator_removed": true}
    }}}));
    assert!(try_known_painless(&mut absent, script));
    assert_eq!(
        absent.get("crowdstrike.alert.is_synthetic_quarantine_disposition"),
        Some(&json!(false))
    );
}

/// Verbatim from `crowdstrike/data_stream/vulnerability`: a vendor severity
/// in caps becomes the title case ECS wants.
#[test]
fn a_shouted_severity_becomes_title_case() {
    let script = "if(ctx.json.cve.severity != null && ctx.json.cve.severity != \"\") {\n    \
         def severity_first_char = ctx.json.cve.severity.substring(0, 1);\n    \
         ctx.vulnerability.severity = severity_first_char + \
         ctx.json.cve.severity.substring(1).toLowerCase();\n}";

    for (raw, want) in [("HIGH", "High"), ("CRITICAL", "Critical"), ("low", "low")] {
        let mut event = Event::new(json!({"json": {"cve": {"severity": raw}}}));
        assert!(try_known_painless(&mut event, script), "{raw} unmatched");
        assert_eq!(
            event.get_str("vulnerability.severity"),
            Some(want),
            "raw {raw}"
        );
    }

    // The script's own guard: an empty string writes nothing.
    let mut empty = Event::new(json!({"json": {"cve": {"severity": ""}}}));
    assert!(try_known_painless(&mut empty, script));
    assert!(!empty.has("vulnerability.severity"));
}

/// Verbatim from `crowdstrike/data_stream/identity_protection_assessment`:
/// a 0-1 score scaled to the 0-100 ECS field.
#[test]
fn a_unit_risk_score_scales_to_the_ecs_hundred() {
    let script = "ctx.event = ctx.event ?: [:];\n\
         double v = ((Number) ctx.event.risk_score).doubleValue();\n\
         ctx.event.risk_score_norm = (long) Math.round(v * 100.0);";

    for (raw, want) in [(0.75, 75), (0.660_000_000_000_000_1, 66), (0.25, 25)] {
        let mut event = Event::new(json!({"event": {"risk_score": raw}}));
        assert!(try_known_painless(&mut event, script), "{raw} unmatched");
        assert_eq!(
            event.get_i64("event.risk_score_norm"),
            Some(want),
            "raw {raw}"
        );
    }
}

/// Verbatim from the same pipeline: an overall level named in caps picks
/// the ECS severity number.
#[test]
fn an_overall_score_level_picks_its_severity() {
    let script = "ctx.event = ctx.event ?: [:];\n\
         def level = ctx.crowdstrike.idp.security_assessment.overall_score_level;\n\
         if (level instanceof String) {\n  def u = level.toUpperCase();\n  \
         if (u == 'LOW' || u == 'NEUTRAL') {\n    ctx.event.severity = 21;\n  \
         } else if (u == 'MEDIUM') {\n    ctx.event.severity = 47;\n  \
         } else if (u == 'HIGH') {\n    ctx.event.severity = 73;\n  \
         } else if (u == 'CRITICAL') {\n    ctx.event.severity = 99;\n  }\n}";

    for (level, want) in [("LOW", 21), ("NEUTRAL", 21), ("MEDIUM", 47), ("HIGH", 73)] {
        let mut event = Event::new(json!({"crowdstrike": {"idp": {"security_assessment": {
            "overall_score_level": level
        }}}}));
        assert!(try_known_painless(&mut event, script), "{level} unmatched");
        assert_eq!(event.get_i64("event.severity"), Some(want), "level {level}");
    }
}

/// Verbatim from the same pipeline: the vendor's camelCase factor list
/// rebuilt under `snake_case` keys, replacing the original.
#[test]
fn the_factor_list_is_rebuilt_under_snake_case_keys() {
    let script = "def sa = ctx.crowdstrike.idp.security_assessment;\n\
         def factors = sa.remove('assessmentFactors');\n\
         if (!(factors instanceof List)) {\n  return;\n}\n\
         def out = new ArrayList();\n\
         for (def f : factors) {\n  if (!(f instanceof Map)) {\n    continue;\n  }\n  \
         def m = new HashMap();\n  if (f.containsKey('riskFactorType')) {\n    \
         m.put('risk_factor_type', f.get('riskFactorType'));\n  }\n  \
         if (f.containsKey('likelihood')) {\n    m.put('likelihood', f.get('likelihood'));\n  }\n  \
         if (f.containsKey('severity')) {\n    m.put('severity', f.get('severity'));\n  }\n  \
         out.add(m);\n}\nsa.put('assessment_factors', out);";

    let mut event = Event::new(json!({"crowdstrike": {"idp": {"security_assessment": {
        "assessmentFactors": [
            {"likelihood": "HIGH", "riskFactorType": "WEAK_PASSWORD_POLICY", "severity": "LOW"}
        ]
    }}}}));
    assert!(try_known_painless(&mut event, script));

    assert_eq!(
        event.get("crowdstrike.idp.security_assessment.assessment_factors"),
        Some(&json!([{
            "risk_factor_type": "WEAK_PASSWORD_POLICY",
            "likelihood": "HIGH",
            "severity": "LOW"
        }]))
    );
    // `remove` takes the camelCase key with it.
    assert!(!event.has("crowdstrike.idp.security_assessment.assessmentFactors"));
}

/// Verbatim from `pipelines/checkpoint/firewall/default.yml`, whose factor
/// carries Java's long suffix.
#[test]
fn a_scale_factor_may_carry_the_java_long_suffix() {
    let script = "ctx.event.duration = ctx.event.duration * 1000000000L";

    let mut event = Event::new(json!({ "event": { "duration": 1931 } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_i64("event.duration"), Some(1_931_000_000_000));
}

#[test]
fn converts_a_duration_from_seconds_to_nanoseconds() {
    let mut event = Event::new(json!({ "fortinet": { "firewall": { "duration": 42 } } }));
    assert!(try_known_painless(&mut event, DURATION_NANOS));
    assert_eq!(event.get_i64("event.duration"), Some(42_000_000_000));
}

/// Both operands come off the wire. A vendor reporting a nonsense count
/// must cost a saturated total, not a debug panic or a negative release
/// one -- these are byte counts a dashboard sums.
#[test]
fn a_nonsense_byte_count_saturates_rather_than_wrapping() {
    let mut event = Event::new(json!({
        "source": { "bytes": i64::MAX },
        "destination": { "bytes": 1 },
    }));

    assert!(try_known_painless(&mut event, SUM_BYTES));
    assert_eq!(event.get_i64("network.bytes"), Some(i64::MAX));
}

#[test]
fn a_nonsense_duration_saturates_rather_than_wrapping() {
    let mut event = Event::new(json!({ "fortinet": { "firewall": { "duration": i64::MAX } } }));

    assert!(try_known_painless(&mut event, DURATION_NANOS));
    assert_eq!(event.get_i64("event.duration"), Some(i64::MAX));
}

/// A duration script whose field name this code cannot read is NOT
/// handled. Counting it would inflate the coverage figure with scripts
/// nothing actually ran.
#[test]
fn an_unreadable_duration_script_is_not_counted_as_handled() {
    let mut event = Event::new(json!({}));
    let script = "ctx.event.duration = Long.parseLong(something) * 1000000000";
    assert!(!try_known_painless(&mut event, script));
}

/// The vendor field is often a string, because it came out of a grok.
#[test]
fn a_string_duration_converts_too() {
    let mut event = Event::new(json!({ "fortinet": { "firewall": { "duration": "7" } } }));
    assert!(try_known_painless(&mut event, DURATION_NANOS));
    assert_eq!(event.get_i64("event.duration"), Some(7_000_000_000));
}

/// The mapping is read out of the ladder, not transcribed into Rust --
/// a hand-written copy is what goes stale when a vendor adds a protocol.
#[test]
fn an_equality_ladder_assigns_the_matching_arm() {
    for (iana, transport) in [("0", "hopopt"), ("6", "tcp"), ("17", "udp")] {
        let mut event = Event::new(json!({ "network": { "iana_number": iana } }));
        assert!(try_known_painless(&mut event, IANA_LADDER));
        assert_eq!(event.get_str("network.transport"), Some(transport));
    }
}

/// The grok types this capture as a long, so the ladder has to compare
/// the number's text -- Painless is doing the same widening.
#[test]
fn an_equality_ladder_reads_a_numeric_subject() {
    let mut event = Event::new(json!({ "network": { "iana_number": 6 } }));
    assert!(try_known_painless(&mut event, IANA_LADDER));
    assert_eq!(event.get_str("network.transport"), Some("tcp"));
}

/// A value no arm names leaves the target alone, rather than taking the
/// last arm or writing a placeholder.
#[test]
fn an_equality_ladder_with_no_matching_arm_writes_nothing() {
    let mut event = Event::new(json!({ "network": { "iana_number": "254" } }));
    assert!(try_known_painless(&mut event, IANA_LADDER));
    assert!(!event.has("network.transport"));
}

/// panw's own "crude `uri_parts`", as the generator emits it.
const SCHEMELESS_URL: &str = r#"Map url = new HashMap();
String url_original = ctx.url.original;
String domainPort = url_original;
url.original = url_original;
if (url_original.contains("/")) {
int idxSlash = url_original.indexOf("/");
domainPort = url_original.substring(0, idxSlash);
}
if (domainPort.indexOf(":") != -1) {
url.domain = domainPort.splitOnToken(":")[0];
}
ctx.url = url;
"#;

#[test]
fn a_schemeless_url_splits_into_domain_path_and_extension() {
    let mut event = Event::new(json!({
        "url": { "original": "lorexx.cn/loader.exe" },
        "destination": {},
    }));

    assert!(try_known_painless(&mut event, SCHEMELESS_URL));
    assert_eq!(event.get("url.domain"), Some(&json!("lorexx.cn")));
    assert_eq!(event.get("url.path"), Some(&json!("/loader.exe")));
    assert_eq!(event.get("url.extension"), Some(&json!("exe")));
    assert_eq!(event.get("destination.domain"), Some(&json!("lorexx.cn")));
}

#[test]
fn a_query_string_is_split_off_the_path() {
    let mut event = Event::new(json!({
        "url": { "original": "lsiu.info/evo/count.php?id=7&v=2" },
    }));

    assert!(try_known_painless(&mut event, SCHEMELESS_URL));
    assert_eq!(event.get("url.path"), Some(&json!("/evo/count.php")));
    assert_eq!(event.get("url.query"), Some(&json!("id=7&v=2")));
    assert_eq!(event.get("url.extension"), Some(&json!("php")));
}

#[test]
fn a_port_is_taken_off_the_domain() {
    let mut event = Event::new(json!({ "url": { "original": "example.com:8080/a" } }));

    assert!(try_known_painless(&mut event, SCHEMELESS_URL));
    assert_eq!(event.get("url.domain"), Some(&json!("example.com")));
    assert_eq!(event.get("url.port"), Some(&json!(8080)));
}

/// The script swallows the parse failure, so a non-numeric port must leave
/// `url.port` unset rather than failing the event or storing the text.
#[test]
fn a_non_numeric_port_leaves_the_port_unset() {
    let mut event = Event::new(json!({ "url": { "original": "example.com:abc/a" } }));

    assert!(try_known_painless(&mut event, SCHEMELESS_URL));
    assert_eq!(event.get("url.domain"), Some(&json!("example.com")));
    assert!(!event.has("url.port"));
}

/// `ctx.url = url` REPLACES the object, so a field already under it goes.
#[test]
fn the_url_object_is_replaced_not_merged() {
    let mut event = Event::new(json!({
        "url": { "original": "example.com/a", "stale": "left over" },
    }));

    assert!(try_known_painless(&mut event, SCHEMELESS_URL));
    assert!(!event.has("url.stale"));
}

/// Painless would throw writing through an absent `ctx.destination`, so
/// inventing one here would produce an object Elastic never emitted.
#[test]
fn an_absent_destination_is_not_created() {
    let mut event = Event::new(json!({ "url": { "original": "example.com/a" } }));

    assert!(try_known_painless(&mut event, SCHEMELESS_URL));
    assert!(!event.has("destination"));
}

/// The `cisco_ios` timezone chain, as the current pipeline writes it: a lookup
/// wrapped in a function, whose branches RETURN rather than assign, and
/// whose last resort sits outside every `if`.
const TZ_CHAIN: &str = r"String get_timezone(def ctx) {
  if (ctx._temp_?.tz != null) {
if (ctx._conf?.tz_map != null) {
  for (def item : ctx._conf.tz_map) {
    if (item.tz_short == ctx._temp_.tz) {
      return item.tz_long;
    }
  }
}
if (ctx._temp_.tz.length() <= 4) {
  return ctx._temp_.tz.toUpperCase();
}
return ctx._temp_.tz;
  }
  if (ctx._conf?.tz_offset != null) {
  ctx.event.timezone = ctx._conf.tz_offset;
  return ctx._conf.tz_offset;
  }
  ctx.event.timezone = 'UTC';
  return 'UTC';
}
def event_timezone = get_timezone(ctx);
";

/// With no timezone on the line and none configured, the last resort runs.
#[test]
fn a_lookup_chain_falls_through_to_its_unguarded_default() {
    let mut event = Event::new(json!({ "_temp_": { "cisco_timestamp": "Jul 14 2023" } }));

    assert!(try_known_painless(&mut event, TZ_CHAIN));
    assert_eq!(event.get("event.timezone"), Some(&json!("UTC")));
}

/// A timezone parsed off the line takes the first branch and RETURNS, so
/// the default must not fire behind it.
#[test]
fn a_guard_that_holds_suppresses_the_default() {
    let mut event = Event::new(json!({ "_temp_": { "tz": "CEST" } }));

    assert!(try_known_painless(&mut event, TZ_CHAIN));
    assert!(!event.has("event.timezone"));
}

/// The configured offset is a fallback ARM, and it beats the default.
#[test]
fn a_configured_offset_wins_over_the_default() {
    let mut event = Event::new(json!({ "_conf": { "tz_offset": "+10:00" } }));

    assert!(try_known_painless(&mut event, TZ_CHAIN));
    assert_eq!(event.get("event.timezone"), Some(&json!("+10:00")));
}

#[test]
fn syslog_priority_decomposes_into_facility_and_severity() {
    // Verbatim from the fortinet transform.
    const PRIORITY: &str = "if (ctx.log?.syslog?.priority != null) {\n  \
         def severity = new HashMap();\n  severity['code'] = ctx.log.syslog.priority&0x7;\n  \
         ctx.log.syslog['severity'] = severity;\n  def facility = new HashMap();\n  \
         facility['code'] = ctx.log.syslog.priority>>3;\n  \
         ctx.log.syslog['facility'] = facility;\n}";

    let mut event = Event::new(json!({ "log": { "syslog": { "priority": 165 } } }));
    assert!(try_known_painless(&mut event, PRIORITY));

    // 165 = local4(20) * 8 + notice(5).
    assert_eq!(event.get_i64("log.syslog.facility.code"), Some(20));
    assert_eq!(event.get_i64("log.syslog.severity.code"), Some(5));

    // The script derives CODES only, so a name is an extra field Elastic
    // never emits -- and every one of them was a diff against the vendor.
    assert!(!event.has("log.syslog.facility.name"));
    assert!(!event.has("log.syslog.severity.name"));
}

/// Cisco nexus derives only the facility here; a `set` processor earlier
/// in the pipeline supplies the severity from the vendor's own field.
#[test]
fn syslog_priority_writes_only_the_half_the_script_names() {
    const FACILITY_ONLY: &str = "ctx.log.syslog.facility = new HashMap();\n\
         ctx.log.syslog.facility.code = (ctx.cisco_nexus.log.priority_number - \
         ctx.event.severity)/8;";

    let mut event = Event::new(json!({
        "cisco_nexus": { "log": { "priority_number": 165 } },
        "event": { "severity": 5 },
    }));
    assert!(try_known_painless(&mut event, FACILITY_ONLY));

    assert_eq!(event.get_i64("log.syslog.facility.code"), Some(20));
    assert!(!event.has("log.syslog.severity.code"));
}

#[test]
fn append_unique_skips_duplicates_and_keeps_order() {
    let mut event = Event::new(json!({
        "dns": { "resolved_ip": ["1.1.1.1", "2.2.2.2", "1.1.1.1"] },
        "related": { "ip": ["1.1.1.1"] },
    }));

    assert!(try_known_painless(&mut event, APPEND_DNS));
    assert_eq!(
        event.get("related.ip"),
        Some(&json!(["1.1.1.1", "2.2.2.2"]))
    );
}

/// The destination array may not exist yet.
#[test]
fn append_unique_creates_the_target_array() {
    let mut event = Event::new(json!({ "dns": { "resolved_ip": ["9.9.9.9"] } }));
    assert!(try_known_painless(&mut event, APPEND_DNS));
    assert_eq!(event.get("related.ip"), Some(&json!(["9.9.9.9"])));
}

/// The predicate 245 of the 351 packages spell: null, empty string, empty
/// collection, in both maps and lists.
fn drop_everything() -> DropPolicy {
    DropPolicy {
        nulls: true,
        empty_strings: true,
        empty_collections: true,
        prune_lists: true,
        sentinels: Vec::new(),
        shallow: false,
    }
}

#[test]
fn drop_empty_removes_nulls() {
    let mut event = Event::new(json!({
        "a": "keep",
        "b": null,
        "c": "",
        "d": {"e": null, "f": "keep"},
        "g": [null, "", "keep"]
    }));
    drop_empty_recursive(&mut event, &drop_everything());
    assert_eq!(event.get_str("a"), Some("keep"));
    assert!(!event.has("b"));
    assert!(!event.has("c"));
    assert!(event.has("d.f"));
    assert!(!event.has("d.e"));
}

/// Verbatim from `pipelines/cisco/asa/default.yml`, which 16 packages
/// share: the predicate is `v == null` and nothing else, so an empty string
/// and an emptied object both stay.
const NULL_ONLY: &str = "void handleMap(Map map) {\n  for (def x : map.values()) {\n    \
    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        \
    handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\n\
    void handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          \
    handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\n\
    handleMap(ctx);";

#[test]
fn a_null_only_predicate_keeps_empty_strings_and_objects() {
    let policy = DropPolicy::read(NULL_ONLY);
    assert_eq!(
        policy,
        DropPolicy {
            nulls: true,
            empty_strings: false,
            empty_collections: false,
            prune_lists: false,
            sentinels: Vec::new(),
            shallow: false,
        }
    );

    let mut event = Event::new(json!({
        "a": "keep",
        "b": null,
        "c": "",
        "d": { "e": null },
        "g": [null, "", "keep"],
    }));

    assert!(try_known_painless(&mut event, NULL_ONLY));
    assert!(!event.has("b"), "a null is still dropped");
    assert_eq!(event.get_str("c"), Some(""), "an empty string is not");
    assert!(event.has("d"), "the emptied object stays");
    assert_eq!(
        event.get("g"),
        Some(&json!([null, "", "keep"])),
        "a list this script never prunes is untouched"
    );
}

/// The common predicate, read off its own text rather than assumed.
#[test]
fn the_full_predicate_reads_as_dropping_everything() {
    let script = "boolean drop(Object o) { if (o == null || o == '') { return true; } \
        else if (o instanceof Map) { ((Map) o).values().removeIf(v -> drop(v)); \
        return ((Map) o).size() == 0; } else if (o instanceof List) { \
        ((List) o).removeIf(v -> drop(v)); return ((List) o).length == 0; } return false; } \
        drop(ctx);";

    assert_eq!(DropPolicy::read(script), drop_everything());
}

#[test]
fn keys_to_snake_case_converts() {
    let mut val = json!({
        "eventType": "login",
        "clientIp": "1.2.3.4",
        "nested": {"displayName": "test"}
    });
    keys_to_snake_case(&mut val, SnakeRule::BeforeEveryUpper);
    assert!(val.get("event_type").is_some());
    assert!(val.get("client_ip").is_some());
    assert!(val.get("eventType").is_none());
}

/// The twenty-line `camelToSnake` / `convertToSnakeCase` pair, cut down to
/// what the matcher keys on plus the apply line it reads the paths from.
const CAMEL_TO_SNAKE: &str = "String camelToSnake(String str) {\n\
    def result = \"\";\n\
    if (Character.isUpperCase(c)) { result += \"_\"; }\n\
    return result;\n\
    }\n\
    def convertToSnakeCase(def obj) {\n\
    if (obj instanceof Map) {\n\
    if (!entry.getKey().contains(\"@\")) {\n\
    String newKey = camelToSnake(entry.getKey());\n\
    newObj[newKey] = convertToSnakeCase(entry.getValue());\n\
    }\n\
    } else if (obj instanceof List) {\n\
    for (item in obj) { newList.add(convertToSnakeCase(item)); }\n\
    }\n\
    }\n";

#[test]
fn camel_to_snake_writes_the_converted_object_to_its_target() {
    let script = format!(
        "{CAMEL_TO_SNAKE}ctx.sentinel_one = ctx.sentinel_one ?: [:];\n\
         ctx.sentinel_one.unified_alert = convertToSnakeCase(ctx.json);\n"
    );
    let mut event = Event::new(json!({
        "json": {"analystVerdict": "UNDEFINED", "detectionSource": {"vendorName": "S1"}}
    }));

    assert!(try_known_painless(&mut event, &script));
    assert_eq!(
        event.get_str("sentinel_one.unified_alert.analyst_verdict"),
        Some("UNDEFINED")
    );
    assert_eq!(
        event.get_str("sentinel_one.unified_alert.detection_source.vendor_name"),
        Some("S1"),
    );
    assert!(event.has("json"), "the source object is not consumed");
}

/// netskope derives its users from the addresses, and NAMES only one.
///
/// Verbatim from `-dev/pipelines/netskope/events/default.yml:1077`.
/// `user.name` was wrong on 27 events and `related.user` on 48.
#[test]
fn addresses_become_related_users_and_name_only_one() {
    let script = "def parts = ctx.user.email;\nif (!(parts instanceof String)) {\n  \
        List l = new ArrayList();\n  for (entry in parts.entrySet()) {\n    \
        l.add(entry.getValue());\n  }\n  List setList = new ArrayList(new HashSet(l));\n  \
        ctx.user.email = setList;\n}\nif (ctx.user.email instanceof List) {\n  \
        def related_users = [];\n  for (def email : ctx.user.email) {\n    \
        if (email.contains('@')) {\n      related_users.add(email.splitOnToken('@')[0])\n    \
        }\n  }\n  if (ctx.related == null) {\n    ctx.related = new HashMap();\n  }\n  \
        ctx.related.user = related_users;\n  if (related_users.length == 1) {\n    \
        ctx.user.name = related_users[0]\n  }\n}";

    // The numbered map is flattened and deduped first, then derived from.
    let mut event = Event::new(json!({ "user": { "email": {
        "0": "test@example.com", "1": "test@example.com", "2": "test@example.com"
    } } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("user.email"), Some(&json!(["test@example.com"])));
    assert_eq!(event.get("related.user"), Some(&json!(["test"])));
    assert_eq!(event.get_str("user.name"), Some("test"));

    // TWO addresses make the name ambiguous, so the vendor leaves it unset.
    let mut two = Event::new(json!({ "user": { "email": ["a@x.com", "b@y.com"] } }));
    assert!(try_known_painless(&mut two, script));
    assert_eq!(two.get("related.user"), Some(&json!(["a", "b"])));
    assert!(!two.has("user.name"), "two addresses name nobody");

    // An entry with no `@` contributes nothing.
    let mut odd = Event::new(json!({ "user": { "email": ["nobody", "c@z.com"] } }));
    assert!(try_known_painless(&mut odd, script));
    assert_eq!(odd.get("related.user"), Some(&json!(["c"])));
    assert_eq!(odd.get_str("user.name"), Some("c"));
}

/// netskope stores a single-valued field as a numbered map and flattens it.
///
/// Verbatim from `-dev/pipelines/netskope/events/default.yml:1060`. Without
/// the dedupe the same mime type landed three times and no event matched.
#[test]
fn a_maps_values_collect_into_a_deduplicated_list() {
    let script = "def parts = ctx.file.mime_type; if (parts != null && parts.size() > 0) {\n  \
        List l = new ArrayList();\n  for (entry in parts.entrySet()) {\n    \
        l.add(entry.getValue());\n  }\n  List setList = new ArrayList(new HashSet(l));\n  \
        ctx.file.mime_type = setList;\n}";

    // Three copies of one value collapse to one.
    let mut event = Event::new(json!({ "file": { "mime_type": {
        "0": "application/pdf", "1": "application/pdf", "2": "application/pdf"
    } } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("file.mime_type"),
        Some(&json!(["application/pdf"]))
    );

    // A field that is already a list, not a map, is left alone.
    let mut listed = Event::new(json!({ "file": { "mime_type": ["text/plain"] } }));
    assert!(try_known_painless(&mut listed, script));
    assert_eq!(listed.get("file.mime_type"), Some(&json!(["text/plain"])));
}

/// `ti_recordedfuture` snake-cases its evidence list with a REGEX rule.
///
/// Verbatim from `-dev/pipelines/ti_recordedfuture/threat/default.yml:143`.
/// A third rule, and neither `SnakeRule` variant: the greedy `[A-Z]+` run and
/// the "no lower-case before it, no match" case are what separate them.
#[test]
fn a_lists_objects_snake_case_by_the_vendors_regex() {
    let script = "Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  \
        def out = [:];\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    \
        def v = entry.getValue();\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    \
        }\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    out.put(k, v);\n  }\n  \
        return out;\n}\nList evidence_details = new ArrayList();\n\
        for (evidence in ctx.json.evidence_details){\n  \
        evidence_details.add(keysToSnakeCase(evidence));\n}\n\
        ctx.json.evidence_details = evidence_details;";

    let mut event = Event::new(json!({ "json": { "evidence_details": [
        { "sightingsCount": 3, "fooBAR": 1, "HTTPStatus": "ok",
          "nested": { "innerValue": 2 },
          "items": [ { "deepKey": 4 }, "left alone" ] }
    ] } }));
    assert!(try_known_painless(&mut event, script));

    let first = &event.get("json.evidence_details").expect("the list")[0];
    assert_eq!(first["sightings_count"], json!(3));
    // The uppercase run is greedy, so this is not `foo_b_a_r`.
    assert_eq!(first["foo_bar"], json!(1));
    // Nothing lower-case precedes the run, so only the case changes.
    assert_eq!(first["httpstatus"], json!("ok"));
    // Recurses into maps, and into maps INSIDE lists.
    assert_eq!(first["nested"]["inner_value"], json!(2));
    assert_eq!(first["items"][0]["deep_key"], json!(4));
    assert_eq!(first["items"][1], json!("left alone"));
}

/// `ti_recordedfuture` totals one member across its evidence list.
///
/// Verbatim from the source's own script. `threat.indicator.sightings` gated
/// 31 of its events.
#[test]
fn a_member_totals_across_a_list() {
    let script = "def sum_sightings_count = 0;\nfor (evidence in ctx.json.evidence_details){\n  \
        if (evidence['sightings_count'] != null){\n    \
        sum_sightings_count += evidence['sightings_count'];\n  }\n}\n\
        ctx.threat.indicator.sightings = sum_sightings_count;";

    // An entry without the member contributes nothing rather than declining.
    let mut event = Event::new(json!({ "json": { "evidence_details": [
        { "sightings_count": 3 }, { "other": 1 }, { "sightings_count": 4 }
    ] } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("threat.indicator.sightings"), Some(&json!(7)));

    // An empty list totals zero, which is what the vendor's `= 0` start means.
    let mut empty = Event::new(json!({ "json": { "evidence_details": [] } }));
    assert!(try_known_painless(&mut empty, script));
    assert_eq!(empty.get("threat.indicator.sightings"), Some(&json!(0)));
}

/// The GUARDED spelling of the same scaling, which `GuardedCopy` claims.
///
/// gitlab's api stream reaches `Rhs::Scaled` rather than `ScaleField`, and
/// that evaluator read its source as an integer too -- so 0.01969 seconds
/// stayed put where Elasticsearch publishes 19,690.
#[test]
fn a_guarded_scale_reads_a_fractional_source() {
    let script = "if (ctx.gitlab?.api?.duration_s != null) {\n  \
        ctx.event.duration = ctx.gitlab.api.duration_s * 1000000;\n}";

    let mut event = Event::new(json!({ "gitlab": { "api": { "duration_s": 0.01969 } } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("event.duration"), Some(&json!(19_690.0)));

    // A whole number keeps integer arithmetic and stays exact.
    let mut whole = Event::new(json!({ "gitlab": { "api": { "duration_s": 3 } } }));
    assert!(try_known_painless(&mut whole, script));
    assert_eq!(whole.get("event.duration"), Some(&json!(3_000_000)));
}

/// gitlab times a request in FRACTIONAL seconds.
///
/// Verbatim from `pipelines/gitlab/production/default.yml:116`. The integer
/// read declined and scaled nothing, so 0.03275 stayed put where
/// Elasticsearch publishes 3.275E7.
#[test]
fn a_scale_reads_a_fractional_source() {
    let script = "ctx.event['duration'] = ctx.event.duration * 1e9;";

    let mut event = Event::new(json!({ "event": { "duration": 0.03275 } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("event.duration"), Some(&json!(32_750_000.0)));

    // A grok leaves numbers as strings, so the string form scales too.
    let mut text = Event::new(json!({ "event": { "duration": "0.00627" } }));
    assert!(try_known_painless(&mut text, script));
    assert_eq!(text.get("event.duration"), Some(&json!(6_270_000.0)));

    // A whole number still takes the integer path and stays exact.
    let mut whole = Event::new(json!({ "event": { "duration": 2 } }));
    assert!(try_known_painless(&mut whole, script));
    assert_eq!(whole.get("event.duration"), Some(&json!(2e9)));
}

/// `jamf_protect` names the telemetry event by WHICH key is populated.
///
/// Verbatim from `pipelines/jamf_protect/telemetry/default.yml:71`. The ECS
/// `event.action` is a key NAME, not a value anywhere in the document.
#[test]
fn the_first_populated_key_names_the_action() {
    let script = "if (ctx.jamf_protect.telemetry.containsKey('event')) {\n  \
        def eventObject = ctx.jamf_protect.telemetry.event;\n  \
        for (def key : eventObject.keySet()) {\n    if (eventObject[key] != null) {\n      \
        if (!ctx.containsKey('event')) {\n        ctx.event = new HashMap();\n      }\n      \
        ctx.event.action = key;\n      break;\n    }\n  }\n}";

    // The first key with a value wins, in the document's own order.
    let mut event = Event::new(json!({
        "jamf_protect": { "telemetry": { "event": {
            "unrelated": null, "exec": { "pid": 1 }, "open": { "path": "/x" }
        } } }
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("event.action"), Some("exec"));

    // Every member null writes nothing rather than an empty action.
    let mut empty = Event::new(json!({
        "jamf_protect": { "telemetry": { "event": { "exec": null } } }
    }));
    assert!(try_known_painless(&mut empty, script));
    assert!(!empty.has("event.action"));
}

/// An arm that reads a field into ECS and DROPS it.
///
/// Verbatim from `pipelines/stormshield/log/default.yml:480`. Keeping `ipv`
/// left a field Elasticsearch does not emit on 13 of stormshield's 44 events.
#[test]
fn a_ladder_arm_removes_what_the_script_removes() {
    let script = "if (ctx.stormshield.ipv == \"4\") {\n    ctx.network.type = \"ipv4\";\n    \
        ctx.stormshield.remove(\"ipv\");\n} else if (ctx.stormshield.ipv == \"6\") {\n    \
        ctx.network.type = \"ipv6\";\n    ctx.stormshield.remove(\"ipv\");\n}";

    let mut four = Event::new(json!({ "stormshield": { "ipv": "4", "logtype": "filter" } }));
    assert!(try_known_painless(&mut four, script));
    assert_eq!(four.get_str("network.type"), Some("ipv4"));
    assert!(!four.has("stormshield.ipv"), "the arm drops what it read");
    // Only the named key goes.
    assert_eq!(four.get_str("stormshield.logtype"), Some("filter"));

    let mut six = Event::new(json!({ "stormshield": { "ipv": "6" } }));
    assert!(try_known_painless(&mut six, script));
    assert_eq!(six.get_str("network.type"), Some("ipv6"));
    assert!(!six.has("stormshield.ipv"));

    // No arm matches, so nothing is written and nothing is removed.
    let mut other = Event::new(json!({ "stormshield": { "ipv": "9" } }));
    assert!(try_known_painless(&mut other, script));
    assert!(!other.has("network.type"));
    assert_eq!(other.get_str("stormshield.ipv"), Some("9"));
}

/// stormshield's fractional session length, narrowed the way Java narrows.
///
/// Verbatim from `pipelines/stormshield/log/default.yml:435`. Two of its
/// events expect exactly 2,147,483,647 -- that is the vendor's `(int)` cast
/// saturating, not an overflow to correct.
#[test]
fn fractional_seconds_scale_to_nanos_and_saturate_at_an_int() {
    let script = "if (ctx.stormshield?.duration != null) {\n    \
        def duration = Float.parseFloat(ctx.stormshield.duration);\n    \
        duration *= 1000000000;\n    if (!ctx.containsKey(\"event\")) {\n        \
        ctx.event = [:];\n    }\n\n    ctx.event.duration = (int)duration;\n    \
        ctx.stormshield.remove(\"duration\");\n}";

    // 0.09 seconds is exact, and the seconds field is CONSUMED.
    let mut event = Event::new(json!({ "stormshield": { "duration": "0.09" } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("event.duration"), Some(&json!(90_000_000)));
    assert!(!event.has("stormshield.duration"));

    // Anything past ~2.147 seconds hits the cast's ceiling.
    let mut long = Event::new(json!({ "stormshield": { "duration": "30" } }));
    assert!(try_known_painless(&mut long, script));
    assert_eq!(long.get("event.duration"), Some(&json!(2_147_483_647)));

    // Zero stays zero rather than going missing.
    let mut zero = Event::new(json!({ "stormshield": { "duration": "0" } }));
    assert!(try_known_painless(&mut zero, script));
    assert_eq!(zero.get("event.duration"), Some(&json!(0)));
}

/// `qualys_gav` spells the same hoist as `putAll`, which MERGES.
///
/// Reading only the assignment form left its whole payload under `json.*` --
/// 525 extra fields and 606 missing on every one of its events.
#[test]
fn camel_to_snake_merges_when_the_script_says_put_all() {
    let script = format!(
        "{CAMEL_TO_SNAKE}ctx.qualys_gav = ctx.qualys_gav ?: [:];\n\
         ctx.qualys_gav.asset = ctx.qualys_gav.asset ?: [:];\n\
         if (ctx.json != null) {{\n  \
         ctx.qualys_gav.asset.putAll(convertToSnakeCase(ctx.json));\n}}\n\
         ctx.remove('json');\n"
    );
    let mut event = Event::new(json!({
        "qualys_gav": {"asset": {"kept": "already here"}},
        "json": {"assetId": 42, "agentInfo": {"agentVersion": "1.2"}}
    }));

    assert!(try_known_painless(&mut event, &script));
    assert_eq!(event.get("qualys_gav.asset.asset_id"), Some(&json!(42)));
    assert_eq!(
        event.get_str("qualys_gav.asset.agent_info.agent_version"),
        Some("1.2")
    );
    // The distinction from the assignment form: what was there SURVIVES.
    assert_eq!(event.get_str("qualys_gav.asset.kept"), Some("already here"));
    assert!(!event.has("json"), "the script removes its source");
}

/// lambda's REPORT metrics: a map copied to a new path with its keys
/// `snake_cased` by a helper the script names itself, so the pattern of the
/// replacement identifies it rather than the helper's name. `MB` is one
/// word to the vendor's regex, not two letters.
#[test]
fn a_snake_cased_map_copy_lands_on_its_target() {
    let script = "String underscore(String s) {\n    \
         def regex = /_?([a-z])([A-Z]+)/;\n    \
         s = regex.matcher(s).replaceAll('$1_$2').toLowerCase();\n    \
         return s\n}\n\n\
         def out = [:];\n\
         for (def item : ctx.parsed.record.metrics.entrySet()) {\n    \
         out[underscore(item.getKey())] = item.getValue();\n}\n\
         ctx.aws.lambda.metrics = out\n";
    let mut event = Event::new(json!({
        "parsed": {"record": {"metrics": {
            "durationMs": 1234.567,
            "billedDurationMs": 1235,
            "memorySizeMB": 256,
            "maxMemoryUsedMB": 79,
        }}}
    }));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get_f64("aws.lambda.metrics.duration_ms"),
        Some(1234.567)
    );
    assert_eq!(
        event.get_i64("aws.lambda.metrics.billed_duration_ms"),
        Some(1235)
    );
    assert_eq!(
        event.get_i64("aws.lambda.metrics.memory_size_mb"),
        Some(256)
    );
    assert_eq!(
        event.get_i64("aws.lambda.metrics.max_memory_used_mb"),
        Some(79)
    );
}

/// `entra_id` rewrites its own object rather than writing somewhere new, and
/// drops the `@odata.*` metadata on the way through.
#[test]
fn camel_to_snake_rewrites_in_place_and_drops_at_keys() {
    let script = format!(
        "{CAMEL_TO_SNAKE}if (ctx.entityanalytics_entra_id?.user != null) {{\n\
         ctx.entityanalytics_entra_id.user = \
         convertToSnakeCase(ctx.entityanalytics_entra_id.user);\n\
         }}\n"
    );
    let mut event = Event::new(json!({
        "entityanalytics_entra_id": {"user": {
            "accountEnabled": true,
            "@odata.type": "#microsoft.graph.user",
        }}
    }));

    assert!(try_known_painless(&mut event, &script));
    assert_eq!(
        event.get("entityanalytics_entra_id.user.account_enabled"),
        Some(&json!(true))
    );
    assert!(
        !event.has("entityanalytics_entra_id.user.accountEnabled"),
        "the camelCase key does not survive beside the snake_case one",
    );
    let user = event.get("entityanalytics_entra_id.user").unwrap();
    assert_eq!(
        user.as_object().unwrap().len(),
        1,
        "the @odata key is dropped, not renamed: {user}"
    );
}

/// Verbatim from osquery's result stream. The loop is a rename spelled out
/// entry by entry, and every processor after it reads the new path.
#[test]
fn a_map_moves_whole_beneath_a_new_parent() {
    let script = "def dict = ['result': new HashMap()];\n\
         for (entry in ctx['json'].entrySet()) {\n\
         dict['result'][entry.getKey()] = entry.getValue();\n\
         }\n\
         ctx['osquery'] = dict;\n\
         ctx.remove('json');";
    let mut event = Event::new(json!({
        "json": {"action": "removed", "columns": {"device": "/dev/disk1s4"}}
    }));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("osquery.result.action"),
        Some(&json!("removed")),
        "{:?}",
        event.as_value(),
    );
    assert_eq!(
        event.get("osquery.result.columns.device"),
        Some(&json!("/dev/disk1s4"))
    );
    assert!(!event.has("json"), "the script removes what it moved");
}

/// Verbatim from kolide's auth stream. A single-entry list is BOTH the
/// first and the last entry, and the script's own order says last.
#[test]
fn a_lone_list_entry_is_the_last_one() {
    let script = "def evs = ctx.json.events;\n\
         def ts = ctx.json.timestamp;\n\
         int idx = -1;\n\
         for (int i = 0; i < evs.size(); i++) {\n\
         if (evs[i] instanceof Map && evs[i].timestamp == ts) { idx = i; break; }\n\
         }\n\
         if (idx != -1) {\n\
         if (ctx.event == null) { ctx.event = [:]; }\n\
         if (idx == evs.size() - 1) {\n\
         ctx.event.type = ['end'];\n\
         } else if (idx == 0) {\n\
         ctx.event.type = ['start'];\n\
         } else {\n\
         ctx.event.type = ['info'];\n\
         }\n\
         }";

    for (timestamp, wanted) in [("a", "end"), ("b", "start"), ("c", "info"), ("d", "end")] {
        let mut event = Event::new(json!({
            "json": {
                "timestamp": timestamp,
                "events": if timestamp == "a" {
                    json!([{"timestamp": "a"}])
                } else {
                    json!([{"timestamp": "b"}, {"timestamp": "c"}, {"timestamp": "d"}])
                },
            }
        }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get("event.type"),
            Some(&json!([wanted])),
            "entry {timestamp}",
        );
    }
}

/// A document whose own key is in no entry of the list leaves the field
/// alone -- the script's `idx != -1` guard.
#[test]
fn a_document_outside_the_list_writes_nothing() {
    let script = "def evs = ctx.json.events;\n\
         def ts = ctx.json.timestamp;\n\
         int idx = -1;\n\
         for (int i = 0; i < evs.size(); i++) {\n\
         if (evs[i] instanceof Map && evs[i].timestamp == ts) { idx = i; break; }\n\
         }\n\
         if (idx != -1) {\n\
         if (idx == evs.size() - 1) {\n\
         ctx.event.type = ['end'];\n\
         } else if (idx == 0) {\n\
         ctx.event.type = ['start'];\n\
         } else {\n\
         ctx.event.type = ['info'];\n\
         }\n\
         }";
    let mut event = Event::new(json!({
        "event": {"type": ["info"]},
        "json": {"timestamp": "z", "events": [{"timestamp": "a"}]},
    }));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("event.type"), Some(&json!(["info"])));
}

/// `sentinel_one`'s rule breaks the word wherever the PREVIOUS character was
/// not uppercase, which a digit satisfies.
#[test]
fn camel_to_snake_breaks_after_a_digit() {
    let converted = camel_map_to_snake(
        &json!({"cve2021Id": 1, "HTTPServer": 2}),
        SnakeRule::AfterNonUpper,
        true,
    );
    assert!(converted.get("cve2021_id").is_some(), "{converted}");
    assert!(converted.get("httpserver").is_some(), "{converted}");
}

/// `jupiter_one`'s copy of the helper diverged: it breaks only after a
/// LOWERCASE character and removes the object it converted. Its payload
/// carries keys with a literal dot, so under the other rule every one of
/// them gained an underscore Elastic does not write.
#[test]
fn camel_to_snake_after_a_lowercase_only_and_removes_the_source() {
    let script = "String camelToSnake(String str) {\n\
         def result = \"\";\n\
         for (int i = 0; i < str.length(); i++) {\n\
         char c = str.charAt(i);\n\
         if (Character.isUpperCase(c)) {\n\
         if (i > 0 && Character.isLowerCase(str.charAt(i - 1))) {\n\
         result += \"_\";\n\
         }\n\
         result += Character.toLowerCase(c);\n\
         } else {\n\
         result += c;\n\
         }\n\
         }\n\
         return result;\n\
         }\n\
         def convertToSnakeCase(def obj) {\n\
         if (obj instanceof Map) {\n\
         def newObj = [:];\n\
         for (entry in obj.entrySet()) {\n\
         String newKey = camelToSnake(entry.getKey());\n\
         newObj[newKey] = convertToSnakeCase(entry.getValue());\n\
         }\n\
         return newObj;\n\
         } else if (obj instanceof List) {\n\
         def newList = [];\n\
         for (item in obj) {\n\
         newList.add(convertToSnakeCase(item));\n\
         }\n\
         return newList;\n\
         } else {\n\
         return obj;\n\
         }\n\
         }\n\
         ctx.jupiter_one = ctx.jupiter_one ?: [:];\n\
         if (ctx.json != null) {\n\
         ctx.jupiter_one.asset = convertToSnakeCase(ctx.json);\n\
         }\n\
         ctx.remove('json');";
    let mut event = Event::new(json!({
        "json": {"properties": {"tag.AccountName": ["test"], "webLink": "u"}}
    }));

    assert!(try_known_painless(&mut event, script));
    let properties = event.get("jupiter_one.asset.properties").unwrap();
    assert_eq!(
        properties.get("tag.account_name"),
        Some(&json!(["test"])),
        "a dot is not a lowercase character, so the word does not break there: {properties}",
    );
    assert_eq!(properties.get("web_link"), Some(&json!("u")));
    assert!(!event.has("json"), "the script removes what it converted");
}

/// A value map written as an if/else-if chain with `.put()` as the write.
/// Verbatim from `zscaler_zia/firewall`, whose device OS table is spelled
/// this way rather than as params.
#[test]
fn a_literal_value_map_writes_through_put() {
    let script = "String osType = ctx.zscaler_zia.firewall.device.os.type;\n\
        if (ctx.host == null) {\n    Map map = new HashMap();\n    ctx.put('host', map);\n}\n\
        if (ctx.host?.os == null) {\n    Map map = new HashMap();\n    ctx.host.put('os', map);\n}\n\
        if (osType == 'iOS') {\n   ctx.host.os.put('type', 'ios');\n}\n\
        else if (osType == 'Android OS') {\n   ctx.host.os.put('type', 'android');\n}\n\
        else if (osType == 'Windows OS') {\n   ctx.host.os.put('type', 'windows');\n}\n";

    let mut event = Event::new(json!({
        "zscaler_zia": {"firewall": {"device": {"os": {"type": "iOS"}}}}
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("host.os.type"), Some("ios"));

    let mut event = Event::new(json!({
        "zscaler_zia": {"firewall": {"device": {"os": {"type": "Android OS"}}}}
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("host.os.type"), Some("android"));
}

/// A value the table has no arm for leaves the field alone.
#[test]
fn a_literal_value_map_writes_nothing_for_an_unlisted_value() {
    let script = "String osType = ctx.a.b;\n\
        if (osType == 'iOS') {\n   ctx.host.os.put('type', 'ios');\n}\n\
        else if (osType == 'MAC OS') {\n   ctx.host.os.put('type', 'macos');\n}\n";
    let mut event = Event::new(json!({"a": {"b": "Solaris"}}));
    try_known_painless(&mut event, script);
    assert_eq!(event.get("host.os.type"), None);
}

/// Cut from `pipelines/gcp/audit/default.yml`, tagged "Classify actor and
/// target entities into type-specific fields".
const CLASSIFY: &str = "void addNestedValue(def currentCtx, String path, def value) {\n\
    return;\n\
    }\n\
    if (ctx.actor?.entity?.id instanceof List) {\n\
    for (def actorId : ctx.actor.entity.id) {\n\
    String actor = actorId.toString();\n\
    if (actor.startsWith(\"serviceAccount:\") || actor.contains(\".gserviceaccount.com\")) {\n\
    addNestedValue(ctx, \"service.entity.id\", actor);\n\
    }\n\
    else if (actor.startsWith(\"user:\") || (actor.contains(\"@\") && \
    !actor.contains(\".gserviceaccount.com\"))) {\n\
    addNestedValue(ctx, \"user.entity.id\", actor);\n\
    }\n\
    else {\n\
    addNestedValue(ctx, \"entity.id\", actor);\n\
    }\n\
    }\n\
    }";

#[test]
fn each_member_lands_in_the_path_its_own_text_earns() {
    let mut event = Event::new(json!({"actor": {"entity": {"id": [
        "serviceAccount:svc@project.iam.gserviceaccount.com",
        "user@mycompany.com",
        "projects/foo/workloadIdentityPools/bar",
    ]}}}));

    assert!(try_known_painless(&mut event, CLASSIFY));
    assert_eq!(
        event.get("service.entity.id"),
        Some(&json!([
            "serviceAccount:svc@project.iam.gserviceaccount.com"
        ]))
    );
    assert_eq!(
        event.get("user.entity.id"),
        Some(&json!(["user@mycompany.com"])),
        "an `@` that is not a service account is a user"
    );
    assert_eq!(
        event.get("entity.id"),
        Some(&json!(["projects/foo/workloadIdentityPools/bar"])),
        "the trailing else is the catch-all"
    );
}

/// Cut from `pipelines/aws/cloudtrail/default.yml`, both scripts.
const FLATTENED: &str = "ctx._conf.keep_flattened_duplicates = ctx._conf.retain == null ||\n\
    ctx._conf.retain.contains('all');";
const DUPLICATE: &str = "if (ctx.json?.requestParameters != null) {\n\
    ctx.aws.cloudtrail.request_parameters = ctx.json.requestParameters.toString();\n\
    if (ctx._conf.keep_flattened_duplicates) {\n\
    ctx.aws.cloudtrail.flattened.request_parameters = ctx.json.requestParameters;\n\
    }\n\
    }";

/// A Painless map renders as Java's `{k=v, k=v}`, not as JSON.
#[test]
fn a_rendered_object_is_kept_beside_the_object() {
    let mut event = Event::new(json!({"json": {"requestParameters": {
        "principal": "sns.amazonaws.com",
        "functionName": "cloudtrail-events-test",
    }}}));

    assert!(try_known_painless(&mut event, FLATTENED));
    assert_eq!(
        event.get("_conf.keep_flattened_duplicates"),
        Some(&json!(true)),
        "no `retain` means keep"
    );

    assert!(try_known_painless(&mut event, DUPLICATE));
    assert_eq!(
        event.get_str("aws.cloudtrail.request_parameters"),
        Some("{principal=sns.amazonaws.com, functionName=cloudtrail-events-test}")
    );
    assert_eq!(
        event.get_str("aws.cloudtrail.flattened.request_parameters.principal"),
        Some("sns.amazonaws.com")
    );
}

/// A `retain` the flag does not recognise means no flattened copy, and
/// the rendered one is written either way.
#[test]
fn an_unrecognised_retain_keeps_no_duplicate() {
    let mut event = Event::new(json!({
        "_conf": {"retain": "keyword"},
        "json": {"requestParameters": {"principal": "sns.amazonaws.com"}},
    }));

    assert!(try_known_painless(&mut event, FLATTENED));
    assert!(try_known_painless(&mut event, DUPLICATE));
    assert_eq!(
        event.get_str("aws.cloudtrail.request_parameters"),
        Some("{principal=sns.amazonaws.com}")
    );
    assert_eq!(event.get("aws.cloudtrail.flattened"), None);
}

/// Cut from `pipelines/gcp/audit/default.yml` to the three argument
/// patterns: a `ctx.` path, a member of a bound local, and a member of a
/// loop variable over a list.
const ENTITIES: &str = "void addValue(Set entities, def value) {\n\
    if (value != null && value != \"\") { entities.add(value); }\n\
    }\n\
    TreeSet entities = new TreeSet();\n\
    addValue(entities, ctx.json.protoPayload.resourceName);\n\
    HashMap authInfo = ctx.json?.protoPayload?.authenticationInfo ?: new HashMap();\n\
    addValue(entities, authInfo.principalEmail);\n\
    for (def i: ctx.json.protoPayload.delegates) {\n\
    addValue(entities, i.principalSubject);\n\
    }\n\
    if (entities.size() > 0) {\n\
    ctx.related = ctx.related ?: [:];\n\
    ctx.related.entity = entities;\n\
    }";

#[test]
fn entities_collect_sorted_unique_and_non_empty() {
    let mut event = Event::new(json!({"json": {"protoPayload": {
        "resourceName": "projects/elastic-beats",
        "authenticationInfo": {"principalEmail": "xxx@xxx.xxx"},
        "delegates": [
            {"principalSubject": "serviceAccount:a"},
            {"principalSubject": ""},
            {"principalSubject": "serviceAccount:a"},
        ],
    }}}));

    assert!(try_known_painless(&mut event, ENTITIES));
    assert_eq!(
        event.get("related.entity"),
        Some(&json!([
            "projects/elastic-beats",
            "serviceAccount:a",
            "xxx@xxx.xxx"
        ])),
        "sorted, unique, and nothing empty"
    );
}

/// Nothing to collect leaves the field alone rather than writing an empty
/// list, which is what the script's own `entities.size() > 0` says.
#[test]
fn no_entities_writes_no_list() {
    let mut event = Event::new(json!({"json": {}}));
    assert!(try_known_painless(&mut event, ENTITIES));
    assert_eq!(event.get("related.entity"), None);
}

/// Verbatim from `pipelines/gcp/dns/default.yml`, cut to the lines the
/// matcher keys on.
const RDATA: &str = "def rdata = ctx.gcp.dns.rdata;\n\
    def dns_answers = [];\n\
    def answer_parts = /\\t/.split(rdata_answers[i]);\n\
    def name = answer_parts[0];\n\
    def ttl = Long.parseLong(answer_parts[1]);\n\
    dns_answers.add([\"name\": name]);\n\
    ctx.dns.answers = dns_answers;";

#[test]
fn dns_rdata_columns_become_answers() {
    let mut event = Event::new(json!({"gcp": {"dns": {"rdata": concat!(
        "elastic.co.\t300\tIN\ta\t127.0.0.1\n",
        "elastic.co.\t21600\tIN\tns\tns-1168.awsdns-18.org."
    )}}}));

    assert!(try_known_painless(&mut event, RDATA));
    assert_eq!(
        event.get("dns.answers"),
        Some(&json!([
            {"name": "elastic.co", "ttl": 300, "class": "IN", "type": "A", "data": "127.0.0.1"},
            {"name": "elastic.co", "ttl": 21600, "class": "IN", "type": "NS",
             "data": "ns-1168.awsdns-18.org"},
        ]))
    );
}

/// A trailing `...` is the vendor saying the list was cut short, and is
/// not an answer.
#[test]
fn a_truncated_rdata_list_drops_its_last_line() {
    let mut event = Event::new(json!({"gcp": {"dns": {"rdata":
        "elastic.co.\t300\tIN\ta\t127.0.0.1\n..."}}}));

    assert!(try_known_painless(&mut event, RDATA));
    let Some(Value::Array(answers)) = event.get("dns.answers") else {
        panic!("no answers")
    };
    assert_eq!(answers.len(), 1);
}

/// An address is a resolved ip, a CNAME is a host, and an MX's host is
/// its SECOND token -- the first is the preference.
#[test]
fn an_answer_set_fans_out_into_the_ecs_lists() {
    let script = "for (answer in ctx.dns.answers) { ctx.related.ip.add(answer.data); }";
    let mut event = Event::new(json!({"dns": {"answers": [
        {"type": "A", "data": "127.0.0.1"},
        {"type": "CNAME", "data": "www.elastic.co"},
        {"type": "MX", "data": "1 aspmx.l.google.com"},
        {"type": "TXT", "data": "v=spf1"},
    ]}}));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("dns.resolved_ip"), Some(&json!(["127.0.0.1"])));
    assert_eq!(event.get("related.ip"), Some(&json!(["127.0.0.1"])));
    assert_eq!(
        event.get("related.hosts"),
        Some(&json!(["www.elastic.co", "aspmx.l.google.com"]))
    );
}

#[test]
fn extract_process_from_cmd() {
    let mut event = Event::new(json!({
        "crowdstrike": {"event": {"CommandLine": "C:\\Windows\\Explorer.EXE /factory"}}
    }));
    extract_process_fields(&mut event, "crowdstrike.event.CommandLine", "process").unwrap();
    assert_eq!(
        event.get_str("process.command_line"),
        Some("C:\\Windows\\Explorer.EXE /factory")
    );
    assert_eq!(
        event.get_str("process.executable"),
        Some("C:\\Windows\\Explorer.EXE")
    );
}

#[test]
fn epoch_to_iso8601() {
    let mut event = Event::new(json!({"ts": 1_536_846_339}));
    epoch_to_timestamp(&mut event, "ts", "@timestamp").unwrap();
    let ts = event.get_str("@timestamp").unwrap();
    assert!(ts.starts_with("2018-09-13"));
}

#[test]
fn known_painless_drop_nulls() {
    let mut event = Event::new(json!({"a": null, "b": "keep"}));
    let script = r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#;
    assert!(try_known_painless(&mut event, script));
    assert!(!event.has("a"));
    assert!(event.has("b"));
}

#[test]
fn email_split_user() {
    let mut event = Event::new(json!({"user": {"id": "john@example.com"}}));
    let script = r#"String[] splitmail = ctx.user.id.splitOnToken("@"); ctx.user.email = ctx.user.id; ctx.user.domain = splitmail[1]; ctx.user.name = splitmail[0];"#;
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("user.email"), Some("john@example.com"));
    assert_eq!(event.get_str("user.name"), Some("john"));
    assert_eq!(event.get_str("user.domain"), Some("example.com"));
}

#[test]
fn email_split_no_at_sign() {
    let mut event = Event::new(json!({"user": {"id": "not-an-email"}}));
    let script = r#"String[] splitmail = ctx.user.id.splitOnToken("@");"#;
    assert!(try_known_painless(&mut event, script));
    // Should not set email/name/domain when no @ present
    assert!(!event.has("user.email"));
}

#[test]
fn email_split_target_user() {
    let mut event = Event::new(json!({"user": {"target": {"id": "admin@corp.io"}}}));
    let script = r#"String[] splitmail = ctx.user.target.id.splitOnToken("@"); ctx.user.target.email = ctx.user.target.id;"#;
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("user.target.email"), Some("admin@corp.io"));
    assert_eq!(event.get_str("user.target.name"), Some("admin"));
}

/// Verbatim from `pipelines/google_workspace/access_transparency/default.yml`:
/// the address is read from `.email`, and each half lands under TWO
/// prefixes rather than the source's own.
#[test]
fn an_email_split_writes_every_target_its_script_names() {
    let script = "String[] splitmail = ctx.source.user.email.splitOnToken('@');\n\
        if (splitmail.length != 2) {\n  return;\n}\n\
        if (ctx.user == null) {\n  ctx.user = new HashMap();\n}\n\
        ctx.user.name = splitmail[0];\nctx.source.user.name = splitmail[0];\n\
        ctx.user.domain = splitmail[1];\nctx.source.user.domain = splitmail[1];";
    let mut event = Event::new(json!({
        "source": { "user": { "email": "foo@bar.com" } }
    }));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("user.name"), Some("foo"));
    assert_eq!(event.get_str("source.user.name"), Some("foo"));
    assert_eq!(event.get_str("user.domain"), Some("bar.com"));
    assert_eq!(event.get_str("source.user.domain"), Some("bar.com"));
    // The script never writes one, so neither do we.
    assert!(!event.has("user.email"));
}

#[test]
fn risk_behaviors_positive() {
    let mut event = Event::new(json!({
        "okta": {"debug_context": {"debug_data": {"flattened": {"behaviors": {
            "New Geo-Location": "POSITIVE",
            "New Device": "NEGATIVE",
            "Velocity": "POSITIVE"
        }}}}}
    }));
    let script = r"if POSITIVE risk_behaviors";
    assert!(try_known_painless(&mut event, script));
    let behaviors = event.get("okta.debug_context.debug_data.risk_behaviors");
    assert!(behaviors.is_some());
    let arr = behaviors.unwrap().as_array().unwrap();
    assert_eq!(arr.len(), 2);
}

#[test]
fn risk_behaviors_none_positive() {
    let mut event = Event::new(json!({
        "okta": {"debug_context": {"debug_data": {"flattened": {"behaviors": {
            "New Device": "NEGATIVE"
        }}}}}
    }));
    let script = r"if POSITIVE risk_behaviors";
    assert!(try_known_painless(&mut event, script));
    // No POSITIVE entries — risk_behaviors should not be set
    assert!(!event.has("okta.debug_context.debug_data.risk_behaviors"));
}

#[test]
fn okta_target_rename_and_extract() {
    let mut event = Event::new(json!({
        "okta": {"target": [
            {"type": "User", "alternateId": "user@test.com", "displayName": "Test User", "id": "001", "detailEntry": {"extra": "removed", "methodTypeUsed": "push"}},
            {"type": "UserGroup", "alternateId": "admins", "displayName": "Admins", "id": "002", "detailEntry": null}
        ]}
    }));
    let script =
        r"def target = ctx.okta.target; alternateId alternate_id displayName display_name okta";
    assert!(try_known_painless(&mut event, script));

    // Check renamed fields
    let target = event.get("okta.target").unwrap().as_array().unwrap();
    let first = target[0].as_object().unwrap();
    assert!(first.contains_key("alternate_id"));
    assert!(first.contains_key("display_name"));
    assert!(!first.contains_key("alternateId"));

    // detailEntry is narrowed in place, keeping its own name.
    let de = first.get("detailEntry").unwrap().as_object().unwrap();
    assert!(de.contains_key("methodTypeUsed"));
    assert!(!de.contains_key("extra"));

    // Check user/group extraction
    assert!(event.has("okta_target_user"));
    assert!(event.has("okta_target_group"));
}

#[test]
fn replace_dots_in_keys_azure_claims() {
    let mut event = Event::new(json!({
        "azure": {"activitylogs": {"identity": {"claims": {
            "http://schemas.microsoft.com/identity/claims/id": "test123",
            "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name": "user"
        }}}}
    }));
    let script = r"if (ctx.azure.activitylogs.identity.claims != null) {\n  ctx.temp_claims = new HashMap();\n  for (String key : ctx.azure.activitylogs.identity.claims.keySet()) {\n    ctx.temp_claims[key.replace('.', '_')] = ctx.azure.activitylogs.identity.claims.get(key);\n  }\n  ctx.azure.activitylogs.identity.claims = ctx.temp_claims; ctx.remove('temp_claims');\n}";
    assert!(try_known_painless(&mut event, script));
    // Verify dots replaced with underscores in claim keys
    let claims = event
        .as_value()
        .pointer("/azure/activitylogs/identity/claims")
        .expect("claims should exist");
    let obj = claims.as_object().expect("claims should be object");
    // Original dotted keys should be replaced
    assert!(!obj.contains_key("http://schemas.microsoft.com/identity/claims/id"));
    assert!(obj.contains_key("http://schemas_microsoft_com/identity/claims/id"));
    assert_eq!(
        obj.get("http://schemas_microsoft_com/identity/claims/id")
            .unwrap(),
        "test123"
    );
}

#[test]
fn azure_event_category_default() {
    let mut event = Event::new(json!({
        "azure": {"activitylogs": {"properties": {}}}
    }));
    let script = r"if (ctx?.azure?.activitylogs?.properties?.eventCategory != null) { ctx.azure.activitylogs.event_category = ctx.azure.activitylogs.properties.eventCategory; } else { ctx.azure.activitylogs.event_category = 'Administrative'; }";
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get_str("azure.activitylogs.event_category"),
        Some("Administrative")
    );
}

#[test]
fn drop_empty_nested_arrays() {
    let mut event = Event::new(json!({
        "keep": "yes",
        "nested": {"arr": [null, "", {"inner": null}]}
    }));
    drop_empty_recursive(&mut event, &drop_everything());
    assert!(event.has("keep"));
    // nested.arr should be empty after removing all null/empty items
    assert!(!event.has("nested"));
}

#[test]
fn keys_to_snake_case_already_snake() {
    let mut val = json!({"already_snake": "yes", "alreadylower": "yes"});
    keys_to_snake_case(&mut val, SnakeRule::BeforeEveryUpper);
    assert!(val.get("already_snake").is_some());
    assert!(val.get("alreadylower").is_some());
}

/// Verbatim from `crowdstrike/alert/elasticsearch/ingest_pipeline/automated_lead.yml`,
/// tagged `set_host_and_process_from_first_threatgraph_indicator`.
const THREATGRAPH_FIRST: &str = r"def indicator = ctx.crowdstrike.alert.threatgraph_indicators[0];\nif (indicator.host_id != null && indicator.host_id != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.id = indicator.host_id;\n}\nif (indicator.hostname != null && indicator.hostname != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.name = indicator.hostname;\n}\nif (indicator.process_id != null && indicator.process_id != '') {\n  ctx.process = ctx.process ?: [:];\n  ctx.process.entity_id = indicator.process_id;\n}";

#[test]
fn threatgraph_first_indicator_probe() {
    let mut event = Event::new(json!({
        "crowdstrike": { "alert": { "threatgraph_indicators": [
            { "host_id": "bbb", "hostname": "host-1.example.local", "process_id": "1778476306425098436" },
        ]}}
    }));
    assert!(try_known_painless(&mut event, THREATGRAPH_FIRST));
    assert_eq!(event.get("host.id"), Some(&json!("bbb")));
    assert_eq!(event.get("host.name"), Some(&json!("host-1.example.local")));
    assert_eq!(
        event.get("process.entity_id"),
        Some(&json!("1778476306425098436"))
    );
}

/// Verbatim from `crowdstrike/alert/elasticsearch/ingest_pipeline/automated_lead.yml`,
/// tagged `set_event_risk_score_and_severity_from_score`.
const SCORE_SEVERITY: &str = r"long score = ctx.crowdstrike.alert.score;\nctx.event = ctx.event ?: [:];\nctx.event.risk_score = (double) score;\nif (score < 40) {\n  ctx.event.severity = 21;\n} else if (score < 60) {\n  ctx.event.severity = 47;\n} else if (score < 80) {\n  ctx.event.severity = 73;\n} else {\n  ctx.event.severity = 99;\n}";

#[test]
fn score_severity_bands_cover_all_four_arms() {
    for (score, severity) in [(21, 21), (40, 47), (65, 73), (95, 99)] {
        let mut event = Event::new(json!({ "crowdstrike": { "alert": { "score": score } } }));
        assert!(try_known_painless(&mut event, SCORE_SEVERITY));
        assert_eq!(
            event.get("event.risk_score"),
            Some(&json!(f64::from(score))),
            "score {score}"
        );
        assert_eq!(
            event.get("event.severity"),
            Some(&json!(severity)),
            "score {score}"
        );
    }
}

/// Verbatim from `crowdstrike/alert/elasticsearch/ingest_pipeline/default.yml`,
/// tagged `reconstruct_has_script_or_module_ioc_from_ioc_context`: crowdstrike
/// only ships `has_script_or_module_ioc` sometimes, and this reconstructs it
/// from `ioc_context` the rest of the time.
const IOC_CONTEXT_FLAG: &str = r"if (ctx.crowdstrike == null) {\n  ctx.crowdstrike = [:];\n}\nif (ctx.crowdstrike.alert == null) {\n  ctx.crowdstrike.alert = [:];\n}\nfor (def c: ctx.crowdstrike.alert.ioc_context) {\n  if (c.type == 'module' || c.type == 'script') {\n    ctx.crowdstrike.alert.has_script_or_module_ioc = true;\n    return;\n  }\n}\nctx.crowdstrike.alert.has_script_or_module_ioc = false;\n";

#[test]
fn ioc_context_flag_true_on_a_script_or_module_hit() {
    let mut event = Event::new(json!({
        "crowdstrike": { "alert": { "ioc_context": [
            { "type": "domain", "value": "example.com" },
            { "type": "script", "value": "bad.ps1" },
        ]}}
    }));
    assert!(try_known_painless(&mut event, IOC_CONTEXT_FLAG));
    assert_eq!(
        event.get("crowdstrike.alert.has_script_or_module_ioc"),
        Some(&json!(true))
    );
}

#[test]
fn ioc_context_flag_false_with_no_script_or_module_entry() {
    let mut event = Event::new(json!({
        "crowdstrike": { "alert": { "ioc_context": [
            { "type": "domain", "value": "example.com" },
            { "type": "ip_address", "value": "198.51.100.10" },
        ]}}
    }));
    assert!(try_known_painless(&mut event, IOC_CONTEXT_FLAG));
    assert_eq!(
        event.get("crowdstrike.alert.has_script_or_module_ioc"),
        Some(&json!(false))
    );
}

/// Verbatim from `crowdstrike/alert/elasticsearch/ingest_pipeline/default.yml`,
/// tagged `script_to_combine_latitude_and_longitude`.
#[test]
fn combine_latitude_and_longitude_probe() {
    let script = r"def location = new HashMap();\nlocation.put('lat', ctx.crowdstrike.alert.location_latitude_as_int);\nlocation.put('lon', ctx.crowdstrike.alert.location_longitude_as_int);\nif(ctx.observer == null) {\n  ctx.put('observer', new HashMap());\n}\nif(ctx.observer.geo == null){\n  ctx.observer.put('geo', new HashMap());\n}\nctx.observer.geo.location = location;";
    let mut event = Event::new(json!({
        "crowdstrike": { "alert": {
            "location_latitude_as_int": 340_726,
            "location_longitude_as_int": -1_182_610,
        }}
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("observer.geo.location.lat"),
        Some(&json!(340_726))
    );
    assert_eq!(
        event.get("observer.geo.location.lon"),
        Some(&json!(-1_182_610))
    );
}

/// Verbatim from `crowdstrike/identity_protection_timeline/default.yml`,
/// tagged `map_entity_identity_fields_by_type`.
const TIMELINE_ENTITY_IDENTITY: &str = "def entity = ctx.crowdstrike.idp.timeline.entity;\ndef entityType = entity.type;\ndef secondary = entity.secondary_display_name;\nif (secondary != null && secondary instanceof String && secondary != '') {\n  int slash = secondary.indexOf('\\\\');\n  if (slash >= 0) {\n    def domain = secondary.substring(0, slash);\n    if (domain != null && domain != '' && entityType == 'USER') {\n      ctx.user = ctx.user ?: [:];\n      ctx.user.domain = domain;\n    }\n  } else if (entityType == 'ENDPOINT') {\n    ctx.host = ctx.host ?: [:];\n    ctx.host.hostname = secondary;\n  }\n}\nif (entityType == 'ENDPOINT') {\n  if (entity.primary_display_name != null) {\n    ctx.host = ctx.host ?: [:];\n    ctx.host.name = entity.primary_display_name;\n  }\n} else if (entityType == 'USER') {\n  if (entity.primary_display_name != null) {\n    ctx.user = ctx.user ?: [:];\n    ctx.user.full_name = entity.primary_display_name;\n  }\n}\n";

#[test]
fn timeline_entity_identity_splits_a_domain_backslash_user() {
    let mut event = Event::new(json!({
        "crowdstrike": { "idp": { "timeline": { "entity": {
            "type": "USER",
            "primary_display_name": "test2",
            "secondary_display_name": "ADFSAORATO.COM\\test2",
        }}}}
    }));
    assert!(try_known_painless(&mut event, TIMELINE_ENTITY_IDENTITY));
    assert_eq!(event.get("user.domain"), Some(&json!("ADFSAORATO.COM")));
    assert_eq!(event.get("user.full_name"), Some(&json!("test2")));
    assert_eq!(event.get("host.hostname"), None);
}

#[test]
fn timeline_entity_identity_endpoint_with_no_backslash() {
    let mut event = Event::new(json!({
        "crowdstrike": { "idp": { "timeline": { "entity": {
            "type": "ENDPOINT",
            "primary_display_name": "DESKTOP-MF7IFEI",
            "secondary_display_name": "desktop-mf7ifei.example.local",
        }}}}
    }));
    assert!(try_known_painless(&mut event, TIMELINE_ENTITY_IDENTITY));
    assert_eq!(
        event.get("host.hostname"),
        Some(&json!("desktop-mf7ifei.example.local"))
    );
    assert_eq!(event.get("host.name"), Some(&json!("DESKTOP-MF7IFEI")));
    assert_eq!(event.get("user.domain"), None);
}

/// Verbatim from `crowdstrike/identity_protection_timeline/default.yml`,
/// tagged `map_entity_accounts_by_type`.
const TIMELINE_ENTITY_ACCOUNTS: &str = "def entity = ctx.crowdstrike.idp.timeline.entity;\ndef entityType = entity.type;\ndef accounts = entity.accounts;\nif (accounts.size() == 0 || !(accounts[0] instanceof Map)) {\n  return;\n}\ndef acc = accounts[0];\nif (entityType == 'ENDPOINT') {\n  ctx.host = ctx.host ?: [:];\n  if (acc.sam_account_name != null) {\n    def sam = acc.sam_account_name;\n    if (sam.endsWith('$')) {\n      sam = sam.substring(0, sam.length() - 1);\n    }\n    if (ctx.host.name == null) {\n      ctx.host.name = sam;\n    }\n  }\n  if (acc.object_sid != null) {\n    ctx.host.id = acc.object_sid;\n  }\n} else if (entityType == 'USER') {\n  ctx.user = ctx.user ?: [:];\n  if (acc.sam_account_name != null) {\n    ctx.user.name = acc.sam_account_name;\n    ctx.user.target = ctx.user.target ?: [:];\n    ctx.user.target.name = acc.sam_account_name;\n  }\n  if (acc.object_sid != null) {\n    ctx.user.id = acc.object_sid;\n  }\n} else {\n  ctx.entity = ctx.entity ?: [:];\n  if (acc.sam_account_name != null) {\n    ctx.entity.target = ctx.entity.target ?: [:];\n    ctx.entity.target.name = acc.sam_account_name;\n  }\n  if (acc.object_sid != null) {\n    ctx.entity.id = acc.object_sid;\n  }\n}\n";

#[test]
fn timeline_entity_accounts_user_branch() {
    let mut event = Event::new(json!({
        "crowdstrike": { "idp": { "timeline": { "entity": {
            "type": "USER",
            "accounts": [
                { "sam_account_name": "test2", "object_sid": "S-1-5-21-1050202168-3263900726-32517219-1106" },
            ],
        }}}}
    }));
    assert!(try_known_painless(&mut event, TIMELINE_ENTITY_ACCOUNTS));
    assert_eq!(event.get("user.name"), Some(&json!("test2")));
    assert_eq!(event.get("user.target.name"), Some(&json!("test2")));
    assert_eq!(
        event.get("user.id"),
        Some(&json!("S-1-5-21-1050202168-3263900726-32517219-1106"))
    );
}

/// The endpoint branch strips a trailing `$` off the SAM name, and only
/// fills `host.name` when nothing has set it already.
#[test]
fn timeline_entity_accounts_endpoint_branch_trims_dollar_and_defers_to_existing_name() {
    let mut event = Event::new(json!({
        "crowdstrike": { "idp": { "timeline": { "entity": {
            "type": "ENDPOINT",
            "accounts": [
                { "sam_account_name": "DESKTOP-MF7IFEI$", "object_sid": "S-1-5-21-1819694714-1249303988-2979750736-1104" },
            ],
        }}}},
        "host": { "name": "already-set" },
    }));
    assert!(try_known_painless(&mut event, TIMELINE_ENTITY_ACCOUNTS));
    assert_eq!(event.get("host.name"), Some(&json!("already-set")));
    assert_eq!(
        event.get("host.id"),
        Some(&json!("S-1-5-21-1819694714-1249303988-2979750736-1104"))
    );
}

/// Verbatim from `crowdstrike/alert/elasticsearch/ingest_pipeline/correlation_detection.yml`,
/// tagged `set_event_provider_from_first_source_product`: unconditional,
/// unlike the guarded siblings below.
#[test]
fn first_element_unconditional_probe() {
    let script = r"ctx.event = ctx.event ?: [:];\nctx.event.provider = ctx.crowdstrike.alert.source_products[0];";
    let mut event = Event::new(json!({
        "crowdstrike": { "alert": { "source_products": ["FirewallLogs PaloAlto", "other"] } }
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("event.provider"),
        Some(&json!("FirewallLogs PaloAlto"))
    );
}

/// Verbatim from the same pipeline, tagged
/// `set_source_domain_from_first_source_host`: guarded, so an existing
/// `source.domain` must survive untouched.
#[test]
fn first_element_guarded_probe() {
    let script = r"ctx.source = ctx.source ?: [:];\nif (ctx.source.domain == null || ctx.source.domain == '') {\n  ctx.source.domain = ctx.crowdstrike.alert.source_hosts[0];\n}";

    let mut fresh = Event::new(json!({
        "crowdstrike": { "alert": { "source_hosts": ["censys.io", "other.example"] } }
    }));
    assert!(try_known_painless(&mut fresh, script));
    assert_eq!(fresh.get("source.domain"), Some(&json!("censys.io")));

    let mut already_set = Event::new(json!({
        "crowdstrike": { "alert": { "source_hosts": ["censys.io"] } },
        "source": { "domain": "keep-me" },
    }));
    assert!(try_known_painless(&mut already_set, script));
    assert_eq!(already_set.get("source.domain"), Some(&json!("keep-me")));
}

/// Verbatim from `pipelines/amazon_security_lake/event/default.yml`, tagged
/// `convert_timestamps_to_milliseconds`. `aws_securityhub/finding` ships
/// the same script over `ctx.aws_securityhub.finding`.
const CONVERT_TIMESTAMPS: &str = r#"def convertToMilliseconds(long timestamp) {\n  if ((long)1e19 - 1 < timestamp) {\n    throw new IllegalArgumentException(\"Timestamp format not recognized: \" + timestamp);\n  } else if ((long)1e16 - 1 < timestamp) {\n    return timestamp / 1000000;  // Convert nanoseconds to milliseconds\n  } else if ((long)1e13 - 1 < timestamp) {\n    return timestamp / 1000;  // Convert microseconds to milliseconds\n  } else if ((long)1e10 - 1 < timestamp) {\n    return timestamp;  // Already in milliseconds, no conversion needed\n  } else {\n    return timestamp * 1000;  // Convert seconds to milliseconds\n  }\n}\ndef processFields(Map fields) {\n  if (fields == null) {\n    return null;\n  }\n  for (entry in fields.entrySet()) {\n    def fieldName = entry.getKey();\n    def fieldValue = entry.getValue();\n    // Check if the field is a nested object (Map)\n    if (fieldValue instanceof Map) {\n      // Recursively process nested objects\n      processFields((Map) fieldValue);\n    } else if (fieldName.endsWith('time') || fieldName.endsWith('_time')) {\n      // If the field name ends with \"time\" or \"_time\" and is a number, convert it\n      if (fieldValue instanceof Number) {\n        fields[fieldName] = convertToMilliseconds(((Number) fieldValue).longValue());\n      }\n    }\n  }\n  return null;\n} processFields(ctx.ocsf);"#;

/// Every magnitude band lands on milliseconds, and the walk recurses into
/// nested objects only -- `test-findings[1]`'s
/// `vulnerabilities[].cve.created_time` sits inside a list and keeps its
/// microseconds, which is what Elasticsearch's own output shows.
#[test]
fn suspected_timestamps_normalise_to_milliseconds_by_magnitude() {
    let mut event = Event::new(json!({ "ocsf": {
        "time": 1_722_327_712_967_320_i64,
        "end_time": 1_722_327_712_967_i64,
        "start_time": 1_722_327_712_967_320_000_i64,
        "logged_time": 1_722_327_712_i64,
        "metadata": { "processed_time": 1_722_327_712_967_320_i64 },
        "original_time": "scope institutions int",
        "timezone_offset": 17,
        "vulnerabilities": [{ "cve": { "created_time": 1_722_327_712_965_081_i64 } }],
    }}));

    assert!(try_known_painless(&mut event, CONVERT_TIMESTAMPS));
    assert_eq!(event.get("ocsf.time"), Some(&json!(1_722_327_712_967_i64)));
    assert_eq!(
        event.get("ocsf.start_time"),
        Some(&json!(1_722_327_712_967_i64))
    );
    assert_eq!(
        event.get("ocsf.metadata.processed_time"),
        Some(&json!(1_722_327_712_967_i64))
    );
    assert_eq!(
        event.get("ocsf.end_time"),
        Some(&json!(1_722_327_712_967_i64)),
        "already milliseconds"
    );
    assert_eq!(
        event.get("ocsf.logged_time"),
        Some(&json!(1_722_327_712_000_i64)),
        "seconds"
    );
    assert_eq!(
        event.get("ocsf.original_time"),
        Some(&json!("scope institutions int")),
        "a string is not a Number"
    );
    assert_eq!(event.get("ocsf.timezone_offset"), Some(&json!(17)));
    assert_eq!(
        event.get("ocsf.vulnerabilities.0.cve.created_time"),
        Some(&json!(1_722_327_712_965_081_i64)),
        "a list member is not walked"
    );
}

/// The walk is rooted where the script says, so the securityhub copy over
/// its own subtree converts and nothing outside it is touched.
#[test]
fn the_walk_is_rooted_where_the_script_names() {
    let script = CONVERT_TIMESTAMPS.replace("ctx.ocsf", "ctx.aws_securityhub.finding");
    let mut event = Event::new(json!({
        "aws_securityhub": { "finding": { "CreatedAt_time": 1_722_327_712_i64 } },
        "ocsf": { "time": 1_722_327_712_i64 },
    }));

    assert!(try_known_painless(&mut event, &script));
    assert_eq!(
        event.get("aws_securityhub.finding.CreatedAt_time"),
        Some(&json!(1_722_327_712_000_i64))
    );
    assert_eq!(event.get("ocsf.time"), Some(&json!(1_722_327_712_i64)));
}

/// Verbatim from the same pipeline, tagged `script_ocsf_resources`: the
/// singular object MOVES onto its plural sibling, so the source key goes
/// with it and the later `foreach ocsf.resources` has something to walk.
#[test]
fn the_singular_resource_moves_onto_the_resources_array() {
    let script = r"ctx.ocsf.resources = [ctx.ocsf.resource];\nctx.ocsf.remove('resource');";
    let mut event = Event::new(json!({ "ocsf": { "resource": {
        "type": "carb le multimedia",
        "owner": { "name": "Dude", "uid": "c6b0192a-4e4c-11ef-90f9-0242ac110005" },
    }}}));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("ocsf.resources"),
        Some(&json!([{
            "type": "carb le multimedia",
            "owner": { "name": "Dude", "uid": "c6b0192a-4e4c-11ef-90f9-0242ac110005" },
        }]))
    );
    assert_eq!(event.get("ocsf.resource"), None);
}

/// A literal holding more than one element is a different pattern, and
/// claiming it would write an unresolvable path -- nothing at all.
#[test]
fn a_multi_element_list_literal_is_not_a_wrap() {
    assert!(parse_wrap_value_in_list("ctx.a.list = [ctx.a.one, ctx.a.two];").is_none());
}

/// The removal has to be the source's own, or a same-named leaf under a
/// different parent would take the source's field with it.
#[test]
fn a_removal_under_another_parent_leaves_the_source() {
    let script = r"ctx.ocsf.resources = [ctx.ocsf.resource];\nctx.other.remove('resource');";
    let mut event = Event::new(json!({ "ocsf": { "resource": { "type": "t" } } }));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("ocsf.resource"), Some(&json!({ "type": "t" })));
}

/// Verbatim from `pipelines/carbon_black_cloud/endpoint_event/default.yml`:
/// allocate the containers, copy the local address, then pick source and
/// destination by the inbound flag. Every statement is one the walk can
/// run, so the whole of it runs rather than none of it.
#[test]
fn a_script_of_only_runnable_statements_runs_whole() {
    let script = "// These allocations may be futile.\n\
        if (ctx.client == null) {\n  ctx.client = new HashMap();\n}\n\
        if (ctx.source == null) {\n  ctx.source = new HashMap();\n}\n\
        if (ctx.destination == null) {\n  ctx.destination = new HashMap();\n}\n\
        ctx.client.ip = ctx.json.local_ip;\n\
        ctx.client.port = ctx.json.local_port;\n\
        if (ctx.json?.netconn_inbound == true) {\n  \
        ctx.destination.ip = ctx.json?.local_ip;\n  \
        ctx.source.ip = ctx.json?.remote_ip;\n\
        } else {\n  \
        ctx.source.ip = ctx.json?.local_ip;\n  \
        ctx.destination.ip = ctx.json?.remote_ip;\n}\n";
    let mut event = Event::new(json!({
        "json": { "local_ip": "127.0.0.1", "local_port": 62909,
                  "remote_ip": "67.43.156.14", "netconn_inbound": true }
    }));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("client.ip"), Some(&json!("127.0.0.1")));
    assert_eq!(event.get("client.port"), Some(&json!(62909)));
    // The inbound arm, which the bare `== true` comparison decides.
    assert_eq!(event.get("destination.ip"), Some(&json!("127.0.0.1")));
    assert_eq!(event.get("source.ip"), Some(&json!("67.43.156.14")));
}

/// Verbatim from `pipelines/mimecast/cloud_integrated_logs/default.yml`,
/// tagged `promote_email_attachments_to_array`: the local-variable spelling
/// removes nothing, because it wraps a field in place.
#[test]
fn the_local_variable_wrap_keeps_its_source() {
    let script = r"def attachments = [];\nattachments.add(ctx.email.attachments);\nctx.email.attachments = attachments;\n";
    let mut event =
        Event::new(json!({ "email": { "attachments": { "file": { "name": "a.pdf" } } } }));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("email.attachments"),
        Some(&json!([{ "file": { "name": "a.pdf" } }]))
    );
}

/// Verbatim from `pipelines/cyberarkpas/audit/audit.yml`: the vendor's own
/// run-counter helper, whose acronym handling none of the other rules has.
#[test]
fn a_snake_case_run_breaks_before_the_last_upper() {
    assert_eq!(
        to_snake_case("MessageID", SnakeRule::AcronymRun),
        "message_id"
    );
    assert_eq!(
        to_snake_case("HTTPServer", SnakeRule::AcronymRun),
        "http_server"
    );
    assert_eq!(
        to_snake_case("IsoTimestamp", SnakeRule::AcronymRun),
        "iso_timestamp"
    );
    // The script's fast path: no uppercase after the first.
    assert_eq!(to_snake_case("Rfc5424", SnakeRule::AcronymRun), "rfc5424");
}

/// Verbatim from `pipelines/cyberarkpas/audit/audit.yml`: the string turned
/// into the boolean Elasticsearch stores.
#[test]
fn a_literal_comparison_replaces_its_field_with_a_boolean() {
    let script = "def value = ctx.cyberarkpas.audit.rfc5424; \
        ctx.cyberarkpas.audit[\"rfc5424\"] = value == 'yes';\n";

    let mut yes = Event::new(json!({ "cyberarkpas": { "audit": { "rfc5424": "yes" } } }));
    assert!(try_known_painless(&mut yes, script));
    assert_eq!(yes.get("cyberarkpas.audit.rfc5424"), Some(&json!(true)));

    let mut no = Event::new(json!({ "cyberarkpas": { "audit": { "rfc5424": "no" } } }));
    assert!(try_known_painless(&mut no, script));
    assert_eq!(no.get("cyberarkpas.audit.rfc5424"), Some(&json!(false)));
}

/// Verbatim from `pipelines/sophos/xg/default.yml`: the log's timezone
/// abbreviation mapped through the config, written back over itself.
#[test]
fn a_record_lookup_writes_back_over_its_subject() {
    let script = "def conf = ctx['_conf'];\nif (conf == null) return;\n\
        def mappings = conf.tz_map;\nif (mappings == null) return;\n\
        def tz_log = ctx._temp_.tz;\nfor (def item : mappings) {\n  \
        if (item.tz_short == tz_log) {\n    ctx._temp_.tz = item.tz_long;\n    \
        break;\n  }\n}";
    let mut event = Event::new(json!({
        "_temp_": { "tz": "IST" },
        "_conf": { "tz_map": [
            { "tz_short": "IST", "tz_long": "Asia/Kolkata" },
            { "tz_short": "AEST", "tz_long": "Australia/Sydney" },
        ]},
    }));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("_temp_.tz"), Some("Asia/Kolkata"));
}

/// The same pattern with a DEFAULT and a different target: the device serial
/// through `_conf.mappings` to `host.name`, falling back to
/// `_conf.default` when no record matches.
#[test]
fn a_record_lookup_falls_back_to_its_default() {
    let script = "def conf = ctx['_conf'];\nif (conf == null) return;\n\
        def serial = ctx.observer.serial_number;\ndef mappings = conf.mappings;\n\
        if (mappings == null) return;\ndef name = conf['default'];\n\
        for (def item : mappings) {\n  if (item.serial_number == serial) {\n    \
        name = item.hostname;\n    break;\n  }\n}\n\
        if (ctx.host == null) {\n  ctx.host = new HashMap();\n}\nctx.host.name = name;";
    let params = json!({
        "default": "defaulttest.local",
        "mappings": [{ "serial_number": "1234567890123456", "hostname": "testhost.local" }],
    });

    let mut unlisted = Event::new(json!({
        "observer": { "serial_number": "C44313350024-P29PUA" }, "_conf": params,
    }));
    assert!(try_known_painless(&mut unlisted, script));
    assert_eq!(unlisted.get_str("host.name"), Some("defaulttest.local"));

    let mut listed = Event::new(json!({
        "observer": { "serial_number": "1234567890123456" }, "_conf": params,
    }));
    assert!(try_known_painless(&mut listed, script));
    assert_eq!(listed.get_str("host.name"), Some("testhost.local"));
}

/// Verbatim from `pipelines/cisco_secure_endpoint/event/default.yml`: two
/// members of one list, one of them folded on the way out.
#[test]
fn a_list_walk_collects_both_of_its_members() {
    let script = "if (ctx.host == null) {\n    ctx.host = new HashMap();\n}\n\
        if (ctx.host.ip == null) {\n    ctx.host.ip = new ArrayList();\n}\n\
        if (ctx.host.mac == null) {\n    ctx.host.mac = new ArrayList();\n}\n\
        for (addr in ctx.cisco.secure_endpoint.computer.network_addresses) {\n    \
        if (addr.ip != null && !addr.ip.isEmpty()) {\n        \
        if (!ctx.host.ip.contains(addr.ip)) {\n            \
        ctx.host.ip.add(addr.ip);\n        }\n    }\n    \
        if (addr.mac != null && !addr.mac.isEmpty()) {\n        \
        def mac_addr = addr.mac.replace(\":\",\"-\").toUpperCase();\n        \
        if (!ctx.host.mac.contains(mac_addr)) {\n            \
        ctx.host.mac.add(mac_addr);\n        }\n    }\n}";
    let mut event = Event::new(json!({ "cisco": { "secure_endpoint": { "computer": {
        "network_addresses": [
            { "ip": "10.10.10.10", "mac": "f9:65:da:22:2a:41" },
            { "ip": "10.10.10.10", "mac": "" },
        ]
    }}}}));

    assert!(try_known_painless(&mut event, script));
    // Deduped, and the empty member contributes nothing.
    assert_eq!(event.get("host.ip"), Some(&json!(["10.10.10.10"])));
    assert_eq!(event.get("host.mac"), Some(&json!(["F9-65-DA-22-2A-41"])));
}

/// The single-member spelling, writing into a path the script names in
/// full rather than a two-segment one.
#[test]
fn a_list_walk_writes_a_deep_target() {
    let script = "if (ctx.related == null) {\n    ctx.related = new HashMap();\n}\n\
        if (ctx.related?.ip == null) {\n    ctx.related.ip = new ArrayList();\n}\n\
        for (addr in ctx.cisco?.secure_endpoint?.computer?.network_addresses) {\n    \
        if (addr.ip != null && !addr.ip.isEmpty()) {\n        \
        if (!ctx.related.ip.contains(addr.ip)) {\n            \
        ctx.related.ip.add(addr.ip);\n        }\n    }\n}";
    let mut event = Event::new(json!({
        "related": { "ip": ["81.2.69.144"] },
        "cisco": { "secure_endpoint": { "computer": {
            "network_addresses": [{ "ip": "10.10.10.10", "mac": "" }]
        }}},
    }));

    assert!(try_known_painless(&mut event, script));
    // The array the pipeline already built is added to, not replaced.
    assert_eq!(
        event.get("related.ip"),
        Some(&json!(["81.2.69.144", "10.10.10.10"]))
    );
}

/// squid's sentinel prune, verbatim from its generated call site.
///
/// The source writes `-` rather than omitting a field, so this one line
/// decides every value derived from the `_tmp` scratch map.
#[test]
fn a_map_prune_drops_every_entry_holding_the_sentinel() {
    let script = r#"ctx._tmp?.values().removeIf(value -> value == \"-\");"#;
    let mut event = Event::new(json!({ "_tmp": {
        "user_name": "-",
        "method": "GET",
        "content_type": "-",
        "url": "http://example.com/",
    }}));

    assert!(try_known_painless(&mut event, script));
    // Survivors keep their insertion order; the sentinels are gone.
    assert_eq!(
        event.get("_tmp"),
        Some(&json!({ "method": "GET", "url": "http://example.com/" }))
    );
}

/// Verbatim from `pipelines/mysql_enterprise/audit/default.yml`: the
/// trailing comma of the array the audit line came from, removed before
/// the JSON parse that would otherwise fail on every event.
#[test]
fn the_last_character_comes_off_a_field() {
    let script = "ctx.message = ctx.message.substring(0, ctx.message.length() - 1);";

    let mut event = Event::new(json!({ "message": "{\"a\":1}," }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("message"), Some("{\"a\":1}"));

    // By character, not by byte, so a multi-byte tail leaves valid UTF-8.
    let mut wide = Event::new(json!({ "message": "ab\u{00e9}" }));
    assert!(try_known_painless(&mut wide, script));
    assert_eq!(wide.get_str("message"), Some("ab"));
}

/// Verbatim from the generated `ti_misp_threat_attributes` call site: the
/// expiry window from an EPOCH base, the LATER of two timestamps winning,
/// and the flag saying the window has already passed. [`IocExpiry`]
/// declines this spelling because the adders' receiver is bound to
/// locals, never to a ctx path.
#[test]
fn a_decay_window_expires_from_the_later_base_and_flags_the_past() {
    let script = r"def dur = ctx._conf.ioc_expiration_duration; def ts = ctx.misp.attribute.timestamp; long tsMillis = ts instanceof Number ? ts.longValue() : Long.parseLong(ts); ZonedDateTime _tmp_decayed_at; ZonedDateTime _tmp_timestamp = ZonedDateTime.ofInstant(Instant.ofEpochMilli(tsMillis * 1000L), ZoneId.of('Z')); ZonedDateTime _tmp_max_time = _tmp_timestamp; if (ctx.misp.attribute.last_seen != null) {\n    ZonedDateTime _tmp_last_seen = ZonedDateTime.parse(ctx.misp.attribute.last_seen);\n    if (_tmp_max_time.isBefore(_tmp_last_seen)) {\n        _tmp_max_time = _tmp_last_seen;\n    }\n} if (dur instanceof String){\n  char time_unit;\n  String time_value;\n  if (dur.length() != 0){\n    time_unit = dur.charAt(dur.length() - 1);\n    time_value = dur.substring(0, dur.length() - 1);\n  }\n  if (time_unit == (char)'d') {\n    _tmp_decayed_at = _tmp_max_time.plusDays(Long.parseLong(time_value));\n  } else if (time_unit == (char)'h') {\n    _tmp_decayed_at = _tmp_max_time.plusHours(Long.parseLong(time_value));\n  } else if (time_unit == (char)'m') {\n    _tmp_decayed_at = _tmp_max_time.plusMinutes(Long.parseLong(time_value));\n  } else {\n    _tmp_decayed_at = _tmp_max_time.plusDays(90L);\n    if (ctx.error == null) {\n      ctx.error = new HashMap();\n    }\n    if (ctx.error.message == null) {\n      ctx.error.message = new ArrayList();\n    }\n    ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n  }\n  ctx.misp.attribute.decayed_at = _tmp_decayed_at;\n} ctx.misp.attribute.decayed = _tmp_decayed_at.isBefore(ZonedDateTime.parse(ctx._tmp.event_ingested))? true : false;\n";

    // The later of the two bases wins, so the window opens at last_seen.
    let mut event = Event::new(json!({
        "_conf": { "ioc_expiration_duration": "30d" },
        "_tmp": { "event_ingested": "2020-01-01T00:00:00Z" },
        "misp": { "attribute": {
            "timestamp": 86400,
            "last_seen": "1970-01-10T00:00:00Z"
        } }
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get_str("misp.attribute.decayed_at"),
        Some("1970-02-09T00:00:00.000Z")
    );
    assert_eq!(event.get("misp.attribute.decayed"), Some(&json!(true)));

    // An unknown unit takes the 90-day default and says so.
    let mut invalid = Event::new(json!({
        "_conf": { "ioc_expiration_duration": "1x" },
        "_tmp": { "event_ingested": "2020-01-01T00:00:00Z" },
        "misp": { "attribute": { "timestamp": 86400 } }
    }));
    assert!(try_known_painless(&mut invalid, script));
    assert_eq!(
        invalid.get_str("misp.attribute.decayed_at"),
        Some("1970-04-02T00:00:00.000Z")
    );
    assert_eq!(
        invalid.get("error.message"),
        Some(&json!([
            "invalid ioc_expiration_duration: using default 90 days"
        ]))
    );
    assert_eq!(invalid.get("misp.attribute.decayed"), Some(&json!(true)));
}

/// Verbatim from `pipelines/ti_opencti/indicator/default.yml`: three
/// levels built with `?:` so the fourth can be appended to.
#[test]
fn a_value_appends_to_a_list_the_script_builds_first() {
    let script = "ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = \
        ctx.threat.indicator ?: [:];\nctx.threat.indicator.as = \
        ctx.threat.indicator.as ?: [];\nctx.threat.indicator.as.add(ctx._tmp_as);\n";

    // Nothing of the path exists: append builds all of it.
    let mut fresh = Event::new(json!({ "_tmp_as": { "number": 64512 } }));
    assert!(try_known_painless(&mut fresh, script));
    assert_eq!(
        fresh.get("threat.indicator.as"),
        Some(&json!([{ "number": 64512 }]))
    );

    // An existing list is appended to, not replaced.
    let mut held = Event::new(json!({
        "_tmp_as": { "number": 2 },
        "threat": { "indicator": { "as": [{ "number": 1 }] } }
    }));
    assert!(try_known_painless(&mut held, script));
    assert_eq!(
        held.get("threat.indicator.as"),
        Some(&json!([{ "number": 1 }, { "number": 2 }]))
    );
}

/// Verbatim from `pipelines/jamf_compliance_reporter/log/default.yml`: a
/// file mode is stored as a number and reads as octal.
#[test]
fn a_file_mode_is_written_back_as_octal() {
    let script = "int temp = (int)ctx.json.file_access_mode;\n\
        ctx.jamf_compliance_reporter.log.attributes.file.access_mode = \
        Integer.toOctalString(temp);\n";

    let mut event = Event::new(json!({ "json": { "file_access_mode": 33188 } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get_str("jamf_compliance_reporter.log.attributes.file.access_mode"),
        Some("100644")
    );

    // Absent source leaves the target alone rather than writing a zero.
    let mut empty = Event::new(json!({ "json": {} }));
    assert!(try_known_painless(&mut empty, script));
    assert!(!empty.has("jamf_compliance_reporter.log.attributes.file.access_mode"));
}

/// The sum still binds, and Painless keeps two integers integral.
#[test]
fn a_field_is_the_sum_of_two_others() {
    let script = "ctx.total = ctx.a + ctx.b;";

    let mut event = Event::new(json!({ "a": 3, "b": 4 }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("total").and_then(serde_json::Value::as_i64),
        Some(7)
    );

    // An absent operand leaves the target alone rather than writing a zero.
    let mut partial = Event::new(json!({ "a": 3 }));
    assert!(try_known_painless(&mut partial, script));
    assert!(!partial.has("total"));
}

/// An addition the parse cannot read is no longer claimed. It used to bind
/// on `" + ctx."` alone, which any script adding two fields anywhere
/// satisfies.
#[test]
fn an_addition_that_is_not_an_assignment_declines() {
    assert!(parse_sum_of_fields("if (ctx.a + ctx.b > 10) { ctx.big = true; }").is_none());
}

/// The chain still binds and still takes the first matching arm.
#[test]
fn a_value_map_writes_the_arm_its_subject_selects() {
    let script = "String osType = ctx.zscaler_zia.firewall.device.os.type;\n\
        if (osType == 'iOS') { ctx.host.os.put('type', 'ios'); }\n\
        else if (osType == 'Android OS') { ctx.host.os.put('type', 'android'); }";

    let mut event = Event::new(json!({ "zscaler_zia": { "firewall": { "device": {
        "os": { "type": "Android OS" } } } } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("host.os.type"), Some("android"));

    // A subject no arm names writes nothing, which is not the same thing as
    // a script the pattern cannot read.
    let mut other = Event::new(json!({ "zscaler_zia": { "firewall": { "device": {
        "os": { "type": "Windows" } } } } }));
    try_known_painless(&mut other, script);
    assert!(!other.has("host.os.type"));
}

/// A recursive walker spells `else if (` and is not a value map. It claimed
/// 91 call sites on that alone and applied to none of them.
#[test]
fn a_branching_script_that_is_not_a_value_map_declines() {
    let script = "def filterMassive(def src) {\n  \
        if (src instanceof Map) {\n    return src;\n  \
        } else if (src instanceof List) {\n    return src;\n  }\n  \
        return src;\n}\ndef out = ctx.qualys;\n";

    assert!(parse_literal_value_map(script).is_none());
}

/// A folded YAML scalar puts the whole preamble on one line, so a
/// line-based read takes the rest of the script as the path.
#[test]
fn a_binding_reads_out_of_a_folded_script() {
    let folded = "if (ctx.a == null) { return; } String v = ctx.b.c; \
        if (v != null) { ctx.d = v; }";
    assert_eq!(
        local_and_ctx_path(folded),
        Some(("v".to_string(), "b.c".to_string()))
    );
}

/// A script opening with a function declaration must not read its header
/// as the local. `ti_opencti` opens `Map hashesToECS(ArrayList hashes) {`
/// before its real binding, and a reader taking the first `def ` and the
/// first ` = ctx.` after it returns the header as the name.
#[test]
fn a_function_header_is_not_a_binding() {
    let script = "Map toEcs(ArrayList hashes) { return hashes; } def out = ctx.a.b;";
    assert_eq!(
        local_and_ctx_path(script),
        Some(("out".to_string(), "a.b".to_string()))
    );
}

/// Both declaration spellings are two words, so both read the same.
#[test]
fn def_and_a_typed_declaration_read_alike() {
    assert_eq!(
        local_and_ctx_path("def x = ctx.a.b;"),
        local_and_ctx_path("String x = ctx.a.b;")
    );
}

/// Verbatim from `jamf_protect_telemetry`, 99 call sites and no corpus
/// capture to catch it.
///
/// The inner guard compares a LOCAL int against -1, which the guard
/// evaluator cannot read, so it parses to a term that never holds. The
/// pattern binds `Basename` first and `GuardedCopy` second, and only
/// `Basename` writes the cut path -- were it to decline, the fallback
/// would write the whole executable as the name.
#[test]
fn a_basename_cut_wins_over_its_never_true_fallback() {
    let script = "if (ctx.process?.executable != null) {\n    \
        int lastSlashIndex = ctx.process.executable.lastIndexOf('/');\n    \
        if (lastSlashIndex != -1) {\n        \
        ctx.process.name = ctx.process.executable.substring(lastSlashIndex + 1);\n    \
        } else {\n        \
        ctx.process.name = ctx.process.executable;\n    }\n}\n";

    let mut event = Event::new(json!({ "process": {
        "executable": "/usr/local/bin/jamf" } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get_str("process.name"),
        Some("jamf"),
        "the fallback ran and wrote the whole path"
    );
}

/// The copy still binds and still runs.
#[test]
fn a_guarded_copy_binds_and_copies() {
    let script = "if (ctx.winlog.event_data.SubjectUserName != null) {\n  \
        ctx.user.name = ctx.winlog.event_data.SubjectUserName;\n}";

    let mut event = Event::new(json!({ "winlog": { "event_data": {
        "SubjectUserName": "derek" } } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("user.name"), Some("derek"));
}

/// A script that only TESTS a field writes nothing whatever the event
/// says, so the pattern declines rather than claiming it and answering false
/// on every event.
#[test]
fn a_guard_with_no_write_declines() {
    let script = "if (ctx.error.message != null) {\n  return;\n}";

    assert!(!Program::parse(script).can_write());
    assert!(parse_single_copy(script).is_none());
}

/// A loop over a ctx-held table still binds and still reads its row.
#[test]
fn a_row_lookup_reads_the_column_its_subject_selects() {
    let script = "for (def row : ctx.tz_map) {\n  \
        if (row.name == ctx.event.timezone) {\n    \
        ctx.event.timezone = row.offset;\n  }\n}";

    let mut event = Event::new(json!({
        "tz_map": [{ "name": "AEST", "offset": "+10:00" }],
        "event": { "timezone": "AEST" }
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("event.timezone"), Some("+10:00"));
}

/// A `for (def ` loop that walks a ctx list without matching a row against
/// a column is not this pattern. It claimed 47 call sites on those two
/// substrings alone and applied to none of them.
#[test]
fn a_loop_with_no_row_match_declines() {
    let script = "for (def entry : ctx.items) {\n  \
        ctx.total = ctx.total + entry.size;\n}";

    assert!(parse_row_lookup(script).is_none());
}

/// Verbatim from `pipelines/ti_opencti/indicator/default.yml`. The bare
/// `.replace(` trigger claimed this script and applied none of it, so
/// `threat.indicator.type` stayed title-cased on all 31 corpus events.
#[test]
fn a_chain_of_string_ops_runs_in_order() {
    let script = "String type = ctx.threat.indicator.type;\n\
        type = type.toLowerCase();\n\
        type = type.replace('stixfile', 'file');\n\
        ctx.threat.indicator.type = type;\n";

    let mut event = Event::new(json!({ "threat": { "indicator": {
        "type": "Windows-Registry-Key" } } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get_str("threat.indicator.type"),
        Some("windows-registry-key")
    );

    // Order is the whole point: `StixFile` only reaches the replace once
    // the lowercase has run, so a chain missing that step writes `StixFile`
    // and one applied backwards writes `stixfile`.
    let mut stix = Event::new(json!({ "threat": { "indicator": {
        "type": "StixFile" } } }));
    assert!(try_known_painless(&mut stix, script));
    assert_eq!(stix.get_str("threat.indicator.type"), Some("file"));

    // Absent source leaves the target alone rather than writing an empty.
    let mut empty = Event::new(json!({ "threat": { "indicator": {} } }));
    assert!(try_known_painless(&mut empty, script));
    assert!(!empty.has("threat.indicator.type"));
}

/// An op off the allowlist takes the WHOLE chain down. `replaceAll` reads a
/// regex, so binding the chain around it would silently change what the two
/// arguments mean.
#[test]
fn an_unreadable_op_declines_the_whole_chain() {
    let script = "String s = ctx.a;\n\
        s = s.toLowerCase();\n\
        s = s.replaceAll('[0-9]+', 'N');\n\
        ctx.b = s;\n";

    assert!(parse_string_ops(script).is_none());

    let mut event = Event::new(json!({ "a": "Host42" }));
    try_known_painless(&mut event, script);
    assert!(!event.has("b"), "a declined chain must not half-apply");
}

/// The single-replace pattern keeps working, and now DECLINES a script it
/// cannot parse instead of claiming it -- which is what let the chain above
/// reach a pattern at all.
#[test]
fn a_single_replace_binds_and_declines_a_chain() {
    let script = "ctx.host.name = ctx.host.hostname.replace('_', '-');";
    let mut event = Event::new(json!({ "host": { "hostname": "web_01" } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("host.name"), Some("web-01"));

    let chain = "String type = ctx.a;\ntype = type.toLowerCase();\n\
        type = type.replace('x', 'y');\nctx.a = type;\n";
    assert!(
        parse_guarded_replace(chain).is_none(),
        "the chain's declaration is not a target path"
    );
}

/// Verbatim from both `pipelines/ti_misp/threat/default.yml` and
/// `.../threat_attributes/default.yml`: the same script with the scrubbed
/// list APPENDED in one and assigned in the other. Claiming only the
/// first write left the marking missing on all 16 tagged events.
#[test]
fn tag_names_are_scrubbed_and_the_tlp_ones_marked() {
    let assigning = "def tags = ctx.misp.tag.stream()\n   \
        .map(t -> t.name.replace('\\\\', '').replace('\"', ''))\n   \
        .collect(Collectors.toList());\ndef tlpTags = tags.stream()\n   \
        .filter(t -> t.startsWith('tlp:'))\n   \
        .map(t -> t.replace('tlp:', '').toUpperCase())\n   \
        .collect(Collectors.toList());\n\nctx.temp_tags = tags;\n\
        ctx.threat.indicator.marking = [ 'tlp': tlpTags ];\n";

    let tagged = json!({
        "misp": { "tag": [
            { "name": "tlp:white" },
            { "name": "mal\\ware\"" },
            { "name": "tlp:green" }
        ] }
    });

    let mut event = Event::new(tagged.clone());
    assert!(try_known_painless(&mut event, assigning));
    // The backslash and the quote come out of every name, and the marking
    // holds the `tlp:` ones with the prefix off and upper-cased.
    assert_eq!(
        event.get("temp_tags"),
        Some(&json!(["tlp:white", "malware", "tlp:green"]))
    );
    assert_eq!(
        event.get("threat.indicator.marking"),
        Some(&json!({ "tlp": ["WHITE", "GREEN"] }))
    );

    // The `threat` stream appends onto a list something else started.
    let appending = "def tags = ctx.misp.tag.stream()\n   \
        .map(t -> t.name.replace('\\\\', '').replace('\"', ''))\n   \
        .collect(Collectors.toList());\ndef tlpTags = tags.stream()\n   \
        .filter(t -> t.startsWith('tlp:'))\n   \
        .map(t -> t.replace('tlp:', '').toUpperCase())\n   \
        .collect(Collectors.toList());\n\nif (ctx.tags == null) {\n  \
        ctx.tags = new ArrayList();\n}\nctx.tags.addAll(tags);\n\
        ctx.threat.indicator.marking = [ 'tlp': tlpTags ];\n";

    let mut held = Event::new(tagged);
    held.set("tags", json!(["preserved"])).unwrap();
    assert!(try_known_painless(&mut held, appending));
    assert_eq!(
        held.get("tags"),
        Some(&json!(["preserved", "tlp:white", "malware", "tlp:green"]))
    );
}

/// Verbatim from `pipelines/symantec_endpoint/log/default.yml`: the
/// numbered column map through a `TreeMap`, so the columns come back in
/// index order with the vendor's single quotes off.
#[test]
fn csv_columns_come_back_in_key_order_unquoted() {
    let script = "def columnArray = [];\ndef sortedMap = new TreeMap();\n\
        sortedMap.putAll(ctx._csv_array);\nsortedMap.forEach((key, value) -> {\n  \
        def v = value;\n  if (v.startsWith(\"'\") && v.endsWith(\"'\"))\n  {\n    \
        v = v.substring(1, v.length() - 1);\n  }\n  columnArray.add(v);\n});\n\
        ctx['_csv_array'] = columnArray;\n";

    // Written out of order, and one member wearing the quotes.
    let mut event = Event::new(json!({
        "_csv_array": { "02": "c", "00": "'Site: Home'", "01": "b" }
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("_csv_array"),
        Some(&json!(["Site: Home", "b", "c"]))
    );
}

/// Verbatim from the same pipeline: the labelled columns become a map and
/// EVERY column contributes to the fingerprint, `NONE` where it had no
/// label. That fingerprint is what names the unlabelled columns later.
#[test]
fn a_labelled_column_becomes_a_pair_and_an_unlabelled_one_a_hole() {
    let script = "def aliases = Collections.unmodifiableMap([\n  \
        'computer': 'computer_name',\n  'domain': 'domain_name',\n  \
        'end_time': 'end',\n  'group_name': 'group',\n  \
        'local': 'local_host_ip',\n  'local_host': 'local_host_ip',\n  \
        'server_name': 'server',\n  'user': 'user_name'\n]);\n\n\
        def keyPattern = /^([a-zA-Z][a-zA-Z0-9 \\(\\)-]{0,28}):(?:\\s(.+)|\\s)?/;\n\
        def keyValue = [:];\ndef fingerprint = [];\nctx._csv_array.forEach(v -> {\n    \
        def m = keyPattern.matcher(v);\n    def key = 'NONE';\n    if (m.matches()) {\n      \
        key = m.group(1).toLowerCase().replace(' ', '_');\n      \
        key = /[\\(\\)]+/.matcher(key).replaceAll('');\n\n      \
        def tmp = aliases[key];\n      if (tmp != null) {\n        key = tmp;\n      }\n\n\n      \
        def value = m.group(2);\n      if (value != null && !value.trim().isEmpty()) {\n        \
        keyValue[key] = value.trim();\n      }\n    }\n\n    fingerprint.add(key);\n    \
        return true;\n});\nif (!keyValue.isEmpty()) {\n  ctx['_csv_map'] = keyValue;\n}\n\
        ctx['_fingerprint'] = String.join(\"|\", fingerprint);\n";

    let mut event = Event::new(json!({
        "_csv_array": [
            "Site: SEPM",
            "Server: srv01",
            "10.0.0.1",
            "Domain: WORKGROUP",
            "Admin:"
        ]
    }));
    assert!(try_known_painless(&mut event, script));

    // `domain` is aliased to `domain_name`; a bare `Admin:` carries no
    // value, so it keys the fingerprint without writing a pair.
    assert_eq!(
        event.get("_csv_map"),
        Some(&json!({
            "site": "SEPM",
            "server": "srv01",
            "domain_name": "WORKGROUP"
        }))
    );
    assert_eq!(
        event.get_str("_fingerprint"),
        Some("site|server|NONE|domain_name|admin")
    );
}

/// Verbatim from the same pipeline: `1e9` is a DOUBLE in Painless, so the
/// product is one too and Elasticsearch publishes `600000000000.0`.
#[test]
fn a_floating_factor_scales_to_a_double() {
    let script = "ctx.event['duration'] = ctx.event.duration * 1e9;";

    let mut event = Event::new(json!({ "event": { "duration": 600 } }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("event.duration"), Some(&json!(600_000_000_000.0)));

    // The long spelling still writes a long.
    let mut long = Event::new(json!({ "event": { "duration": 600 } }));
    assert!(try_known_painless(
        &mut long,
        "ctx.event['duration'] = ctx.event.duration * 1000000000L;"
    ));
    assert_eq!(
        long.get("event.duration"),
        Some(&json!(600_000_000_000_i64))
    );
}

/// Verbatim from `pipelines/suricata/eve/default.yml`: a list
/// deduplicated and then UNWRAPPED when one member is left, which is what
/// makes `destination.domain` a bare string where the answers agreed.
#[test]
fn a_deduplicated_list_unwraps_to_its_last_member() {
    let script = "def domain = ctx.destination?.domain; if (domain instanceof Collection) {\n\n\n  \
        domain = domain.stream().distinct().collect(Collectors.toList());\n  \
        if (domain.length == 1) {\n    domain = domain[0];\n  }\n  \
        ctx.destination.domain = domain;\n}\n";

    // Every member the same collapses to the bare value, not a list.
    let mut one = Event::new(json!({ "destination": { "domain": ["a.example", "a.example"] } }));
    assert!(try_known_painless(&mut one, script));
    assert_eq!(one.get("destination.domain"), Some(&json!("a.example")));

    // Two survivors stay a list, in first-seen order.
    let mut many =
        Event::new(json!({ "destination": { "domain": ["b.example", "a.example", "b.example"] } }));
    assert!(try_known_painless(&mut many, script));
    assert_eq!(
        many.get("destination.domain"),
        Some(&json!(["b.example", "a.example"]))
    );
}

/// The fortiproxy variant: the same idea as stormshield's loop, with the
/// four rules that make it a different pattern.
#[test]
fn the_fortiproxy_kv_drops_what_the_vendor_drops() {
    let script = r#"ctx[\"_fields_\"] = [:];\ndef kvStart = 0; def kvSplit = 0; def inQuote = false;\nPattern wsPattern = /^\\\"|\\\"$/; Pattern wordPattern = /\\W+/;\nfor (int i = 0, n = ctx[\"message\"].length(); i < n; ++i) {\n  char c = ctx[\"message\"].charAt(i);\n  if (c == (char)'\"') {\n    if (inQuote && i < n - 1 && ctx[\"message\"].charAt(i + 1) != (char)' ') {\n      continue;\n    }\n    inQuote = !inQuote;\n  }\n  if (inQuote) {\n    continue;\n  }\n  if (c == (char)'=') {\n    kvSplit = i;\n  }\n  if (c == (char)' ' || i == n - 1) {\n    if (i != kvStart) {\n      def endIndex = i == n - 1 ? i + 1 : i;\n      def key = ctx[\"message\"].substring(kvStart, kvSplit);\n      def value = wsPattern.matcher(ctx[\"message\"].substring(kvSplit + 1, endIndex)).replaceAll(\"\");\n\n      if (value != \"N/A\" && !wordPattern.matcher(key).find()) {\n        ctx[\"_fields_\"].put(key, value);\n      }\n    }\n\n    kvStart = i + 1;\n    kvSplit = i + 1;\n  }\n}"#;

    let mut event = Event::new(json!({
        "message": r#"date=2023-01-01 msg="a b" skipme=N/A dev-id=x tz=UTC"#,
    }));
    assert!(try_known_painless(&mut event, script));

    assert_eq!(event.get_str("_fields_.date"), Some("2023-01-01"));
    // Only the surrounding quotes come off, and the quoted run is whole.
    assert_eq!(event.get_str("_fields_.msg"), Some("a b"));
    assert_eq!(event.get_str("_fields_.tz"), Some("UTC"));
    // An N/A value is DROPPED, not stored empty.
    assert_eq!(event.get("_fields_.skipme"), None);
    // So is a key holding a non-word character.
    assert_eq!(event.get("_fields_.dev-id"), None);
}

/// Verbatim from `pipelines/stormshield/log/default.yml`, which is that
/// source's WHOLE parse -- the vendor walks the message rather than using
/// a `kv` processor, so nothing downstream of it had any input.
#[test]
fn a_hand_written_kv_split_fills_its_namespace() {
    let script = r#"ctx[\"stormshield\"] = new HashMap();\ndef kvStart = 0; def kvSplit = 0; def kvEnd = 0; def inQuote = false;\nfor (int i = 0, n = ctx[\"message\"].length(); i < n; ++i) {\n  char c = ctx[\"message\"].charAt(i);\n  if (c == (char)'\"') {\n    inQuote = !inQuote;\n  }\n  if (inQuote) {\n    continue;\n  }\n  \n  if (c == (char)'=') {\n    kvSplit = i;\n  }\n  if (c == (char)' ' || (i == n - 1)) {\n    if (kvStart != kvSplit) {\n      def key = ctx[\"message\"].substring(kvStart, kvSplit);\n      def end = i;\n      if (i == n - 1)  {\n          end = n;\n      }\n      def value = ctx[\"message\"].substring(kvSplit + 1, end).replace(\"\\\"\", \"\");\n      ctx[\"stormshield\"][key] = value;\n    }\n\n    kvStart = i + 1;\n    kvSplit = i + 1;\n  }\n}"#;

    // A quoted value keeps its spaces and loses its quotes, and the last
    // pair is closed by the end of the message rather than by a space.
    let mut event = Event::new(json!({
        "message": r#"id=firewall time="2023-01-01 10:00:00" pri=5"#,
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("stormshield.id"), Some("firewall"));
    assert_eq!(
        event.get_str("stormshield.time"),
        Some("2023-01-01 10:00:00")
    );
    assert_eq!(event.get_str("stormshield.pri"), Some("5"));

    // The vendor creates the map before its loop, so a message with no
    // pairs still leaves an empty one behind.
    let mut bare = Event::new(json!({ "message": "nopairs" }));
    assert!(try_known_painless(&mut bare, script));
    assert_eq!(bare.get("stormshield"), Some(&json!({})));
}

/// Verbatim from `pipelines/bitdefender/push_notifications/default.yml`:
/// a lookup whose TABLE is on the document, so no params matcher sees it.
#[test]
fn a_lookup_reads_a_table_the_document_carries() {
    let script = "def conftenants = ctx._tmp.tenants;\n      \
        def orgid = ctx.organization.id;\n      \
        if (conftenants instanceof Map && conftenants.containsKey(orgid)) {\n        \
        ctx.organization.name = conftenants[orgid];\n      }";

    let mut event = Event::new(json!({
        "_tmp": { "tenants": { "abc": "test_events.tld" } },
        "organization": { "id": "abc" },
    }));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get_str("organization.name"), Some("test_events.tld"));

    // An id the table does not carry leaves the name unwritten.
    let mut unlisted = Event::new(json!({
        "_tmp": { "tenants": { "abc": "test_events.tld" } },
        "organization": { "id": "zzz" },
    }));
    assert!(try_known_painless(&mut unlisted, script));
    assert_eq!(unlisted.get("organization.name"), None);
}

/// The firehose classifier's ladder order, which is what its correctness
/// rests on, plus the two readings that are easy to get wrong.
#[test]
fn the_firehose_classifier_names_a_log_type() {
    let dataset = |event: &Event| -> Option<String> {
        let message = event.get_string("message")?;
        let lower = message.to_lowercase();
        super::firehose_dataset(event, &message, &lower).map(str::to_owned)
    };

    // A bracket subscript reads the FLAT key, which no dotted path finds.
    let waf = Event::new(json!({
        "aws.kinesis.name": "aws-waf-logs-stream", "message": "anything",
    }));
    assert_eq!(dataset(&waf).as_deref(), Some("aws.waf"));

    // The resolver test must beat the public one: both are route53, and
    // only the ladder's order keeps them apart.
    let resolver = Event::new(json!({
        "aws.cloudwatch.log_group": "/aws/route53/example",
        "message": "{\"version\":1,\"account_id\":\"1\",\"region\":\"us-east-1\",\
            \"vpc_id\":\"vpc-1\",\"query_timestamp\":\"2023-01-01\"}",
    }));
    assert_eq!(
        dataset(&resolver).as_deref(),
        Some("aws.route53_resolver_logs")
    );

    // A record from the route53 group that fails the pattern test is left
    // UNNAMED -- the vendor's chain stops on the group name.
    let unnamed = Event::new(json!({
        "aws.cloudwatch.log_group": "/aws/route53/example", "message": "nothing useful",
    }));
    assert_eq!(dataset(&unnamed), None);

    // An ELB is known by its first token.
    let elb =
        Event::new(json!({ "message": "https 2023-01-01T00:00:00 app/x 1.2.3.4:1 5.6.7.8:2 0.1" }));
    assert_eq!(dataset(&elb).as_deref(), Some("aws.elb_logs"));
}

/// Quotes stay IN the token, and a quoted run holds its spaces.
#[test]
fn firehose_tokens_keep_a_quoted_run_whole() {
    let tokens = super::firehose_tokens(r#"a "b c" d"#);
    assert_eq!(
        tokens,
        vec!["a".to_owned(), "\"b c\"".to_owned(), "d".to_owned()]
    );
}

/// Verbatim from `pipelines/awsfirehose/metrics/default.yml`: the metric
/// names of the first service map, sorted, for a `fingerprint` to key the
/// document by. Written even when nothing was found, as the vendor does.
#[test]
fn the_first_nested_map_gives_up_its_sorted_key_names() {
    let script = "List metricNames = new ArrayList();\n\
        if (ctx.aws != null && ctx.aws instanceof Map) {\n    \
        for (entry in ctx.aws.entrySet()) {\n        def nestedMap = entry.getValue();\n        \
        if (nestedMap instanceof Map) {\n            def metricsMap = nestedMap.get(\"metrics\");\n            \
        if (metricsMap instanceof Map) {\n                metricNames.addAll(metricsMap.keySet());\n                \
        break;\n            }\n        }\n    }\n}\nCollections.sort(metricNames);\n\
        ctx.aws.metrics_names = metricNames;";

    let mut event = Event::new(json!({ "aws": {
        "cloudwatch": { "not_metrics": { "z": 1 } },
        "firehose": { "metrics": { "Sum": 1, "Average": 2 } },
    }}));
    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("aws.metrics_names"),
        Some(&json!(["Average", "Sum"]))
    );

    // Nothing found still writes the empty list, which is what the
    // fingerprint after it hashes.
    let mut bare = Event::new(json!({ "aws": { "firehose": { "other": 1 } } }));
    assert!(try_known_painless(&mut bare, script));
    assert_eq!(bare.get("aws.metrics_names"), Some(&json!([])));
}

/// Verbatim from `pipelines/ti_eset/apt/default.yml`: an expiry a whole
/// number of days out, with no unit ladder for `IocExpiry` to read.
///
/// The vendor writes TWO spaces after the `=`, and the source is a bracket
/// subscript because `@timestamp` is a name no dotted path can spell.
#[test]
fn an_expiry_lands_a_literal_number_of_days_out() {
    let script = "if (ctx.eset == null) {\n  ctx.eset = new HashMap();\n}\n\
        ctx.eset.valid_until =  ZonedDateTime.parse(ctx['@timestamp']).plusDays(365);";
    let mut event = Event::new(json!({ "@timestamp": "2023-02-14T09:38:10.000Z" }));

    assert!(try_known_painless(&mut event, script));
    // The whole path is the target, not just its last segment.
    assert_eq!(
        event.get_str("eset.valid_until"),
        Some("2024-02-14T09:38:10.000Z")
    );
    assert_eq!(event.get("valid_until"), None);
}

/// The list matcher must not claim the map spelling, nor the reverse.
///
/// The two share the `.removeIf(` opening and are told apart only by the
/// subject, so a receiver that is a list stays with the list matcher.
#[test]
fn a_list_prune_is_not_read_as_a_map_prune() {
    let script = r#"ctx.file.path.removeIf(v -> v == \"-\");"#;
    let mut event = Event::new(json!({ "file": { "path": ["-", "/etc/passwd"] } }));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(event.get("file.path"), Some(&json!(["/etc/passwd"])));
}

/// A prune whose receiver is a LOCAL is a different script.
///
/// Reading back to the nearest `ctx.` would claim a field the script never
/// touches, which is the trap the list matcher's subject check exists for.
#[test]
fn a_map_prune_declines_a_local_receiver() {
    let script = r#"def m = ctx.a.b; m.values().removeIf(value -> value == \"-\");"#;
    assert!(
        !known_patterns(&normalise(script))
            .iter()
            .any(|pattern| matches!(pattern, KnownPattern::RemoveMapValue { .. }))
    );
}

/// Verbatim from `o365_metrics/mailbox_usage_detail`, in the ESCAPED form the
/// call site holds -- one line, `\n` and `\"` unresolved until `normalise`.
///
/// The report's own column headings are the JSON keys, and every `convert`,
/// `rename` and the unguarded `fingerprint` after this script name the
/// rewritten ones.
#[test]
fn a_report_heading_becomes_the_key_the_pipeline_renames() {
    let script = r#"String underscore(String s) {\n  String result = /[ -]/.matcher(s).replaceAll('_').replace(\"/\", \"_\").toLowerCase();\n  String result1 = /[\\ufeff]/.matcher(result).replaceAll('');\n  return /[()]/.matcher(result1).replaceAll('')\n}\n\ndef out = [:];\nfor (def item : ctx.o365.metrics.mailbox.usage.detail.entrySet()) {\n  out[underscore(item.getKey())] = item.getValue();\n}\nctx.o365.metrics.mailbox.usage.detail = out;      \n"#;
    let mut event = Event::new(json!({ "o365": { "metrics": { "mailbox": { "usage": {
        "detail": {
            "Report Refresh Date": "2024-12-15",
            "Deleted Item Quota (Byte)": "32212254720",
            "Prohibit Send/Receive Quota (Byte)": "107374182400",
            "\u{feff}Created Date": "2024-10-22"
        }
    }}}}}));

    assert!(try_known_painless(&mut event, script));
    let detail = event
        .get("o365.metrics.mailbox.usage.detail")
        .and_then(Value::as_object)
        .expect("the rebuilt map replaces the one it read");
    let keys: Vec<&str> = detail.keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        [
            "report_refresh_date",
            "deleted_item_quota_byte",
            "prohibit_send_receive_quota_byte",
            "created_date",
        ],
        "the slash swaps for an underscore, the parentheses and the mark go"
    );
    // The values ride along untouched -- a later `convert` types them.
    assert_eq!(
        detail.get("deleted_item_quota_byte"),
        Some(&json!("32212254720"))
    );
}

/// `o365_metrics`'s other spelling: the control characters come off the key in
/// the LOOP, before the helper the loop calls ever sees it.
///
/// Reading the two spans by their position in the text would run the helper
/// first, because a helper is DEFINED above the loop and CALLED inside it.
#[test]
fn a_csv_heading_loses_its_control_characters_before_the_helper_runs() {
    let script = r"String sanitize(String s) {\n  String t = /[ -]/.matcher(s).replaceAll('_');\n  return /[\\(\\)]/.matcher(t).replaceAll('').toLowerCase();\n}\n\ndef out = [:];\nfor (def item : ctx.json.entrySet()) {\n  // Remove control characters from CSV header\n  String key = /\\p{C}/.matcher(item.getKey()).replaceAll('');\n  // Replace spaces and hyphens with sanitize and convert to lowercase.\n  out[sanitize(key)] = item.getValue();\n}\nctx.json = out;\n";
    let mut event = Event::new(json!({ "json": {
        "Storage Used (Byte)\u{7}": "6370739",
        "Non-Deleted Total Item Count": "8"
    }}));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("json"),
        Some(&json!({
            "storage_used_byte": "6370739",
            "non_deleted_total_item_count": "8"
        }))
    );
}

/// `aws.lambda_logs` breaks the camel case FIRST and stores the rebuilt map at
/// a different path, leaving the one it read where it was.
///
/// The Java match eats an underscore sitting in front of the word break, so
/// this is not the shared `to_snake_case`.
#[test]
fn a_lambda_metric_is_rebuilt_under_a_new_path() {
    let script = r"String underscore(String s) {\n    def regex = /_?([a-z])([A-Z]+)/;\n    s = regex.matcher(s).replaceAll('$1_$2').toLowerCase();\n    String result = /[ -]/.matcher(s).replaceAll('_').toLowerCase();\n    String result1 = /[\\ufeff]/.matcher(result).replaceAll('');\n    return /[()]/.matcher(result1).replaceAll('')\n}\n\ndef out = [:];\nfor (def item : ctx.parsed.record.metrics.entrySet()) {\n    out[underscore(item.getKey())] = item.getValue();\n}\nctx.aws.lambda.metrics = out\n";
    let mut event = Event::new(json!({
        "parsed": { "record": { "metrics": {
            "durationMs": 1.5,
            "memorySizeMB": 128,
            "maxMemoryUsedMB": 74
        }}},
        "aws": { "lambda": {} }
    }));

    assert!(try_known_painless(&mut event, script));
    assert_eq!(
        event.get("aws.lambda.metrics"),
        Some(&json!({
            "duration_ms": 1.5,
            "memory_size_mb": 128,
            "max_memory_used_mb": 74
        }))
    );
    // The script assigns, it does not move: the source stays put.
    assert!(event.get("parsed.record.metrics").is_some());
}

/// A character RANGE is not a set of literals, so the whole script is declined.
///
/// Rewriting a key by only SOME of a helper's steps lands it under a name no
/// later processor reads, which is worse than leaving the script unclaimed.
#[test]
fn a_key_rewrite_declines_a_character_range() {
    let script = "String underscore(String s) {\n  return /[a-z]/.matcher(s).replaceAll('_');\n}\n\
        \ndef out = [:];\nfor (def item : ctx.a.b.entrySet()) {\n  \
        out[underscore(item.getKey())] = item.getValue();\n}\nctx.a.b = out;\n";
    assert!(
        !known_patterns(&normalise(script))
            .iter()
            .any(|pattern| matches!(pattern, KnownPattern::RewriteKeys(_)))
    );
}
