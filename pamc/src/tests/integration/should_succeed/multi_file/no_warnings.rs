use super::*;

#[derive(Clone, Debug)]
struct FileBuilder<'a> {
    id: FileId,
    src: &'a str,
}

struct FileTreeBuilder<'a> {
    root: FileId,
    edges_to_children: Vec<(FileId, &'a str, FileId)>,
}

impl FileTreeBuilder<'_> {
    fn build(self) -> FileGraph {
        let mut graph = FileGraph::from_root(self.root);
        for (parent, name, child) in self.edges_to_children {
            graph
                .add_child(parent, name, child)
                .expect("Failed to add child");
        }
        graph
    }
}

fn expect_success_with_no_warnings(
    graph_builder: FileTreeBuilder,
    file_builders: Vec<FileBuilder>,
) {
    let graph = graph_builder.build();
    let files = file_builders
        .into_iter()
        .map(|builder| {
            let tokens = lex(builder.src).expect("Lexing failed");
            let file = parse_file(tokens, builder.id).expect("Parsing failed");
            simplify_file(file).expect("AST Simplification failed")
        })
        .collect();
    let file_items = bind_files(graph.root(), files, &graph).expect("Binding failed");
    let mut registry = NodeRegistry::empty();
    let file_item_list_id = register_file_items(&mut registry, file_items);

    let file_item_list_id =
        validate_variant_return_types_in_file_items(&registry, file_item_list_id)
            .expect("Variant return type validation failed");
    let file_item_list_id = validate_fun_recursion_in_file_items(&mut registry, file_item_list_id)
        .expect("Fun recursion validation failed");
    let file_item_list_id =
        validate_type_positivity_in_file_items(&mut registry, file_item_list_id)
            .expect("Type positivity validation failed");
    let warnings =
        type_check_file_items(&mut registry, file_item_list_id).expect("Type checking failed");
    assert_eq!(0, warnings.len(), "One or more warnings were emitted");
    let _js_ast = JavaScript::generate_code(&registry, file_item_list_id.raw())
        .expect("Code generation failed");
}

// TODO: Fix
#[ignore]
#[test]
fn factorial() {
    // TODO: Automatically build from directory.
    let pack = FileBuilder {
        id: FileId(0),
        src: include_str!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/factorial/mod.ph"
        ),
    };
    let nat = FileBuilder {
        id: FileId(1),
        src: include_str!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/factorial/nat.ph"
        ),
    };
    let ops = FileBuilder {
        id: FileId(2),
        src: include_str!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/factorial/ops.ph"
        ),
    };
    let unused = FileBuilder {
        id: FileId(3),
        src: include_str!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/factorial/unused.ph"
        ),
    };
    expect_success_with_no_warnings(
        FileTreeBuilder {
            root: pack.id,
            edges_to_children: vec![
                (pack.id, "nat", nat.id),
                (pack.id, "ops", ops.id),
                (pack.id, "unused", unused.id),
            ],
        },
        vec![pack, nat, ops, unused],
    );
}
