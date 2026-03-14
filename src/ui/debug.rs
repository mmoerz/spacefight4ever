use bevy::prelude::*;

pub fn debug_print_ui_tree(
    roots: Query<Entity, Without<ChildOf>>,
    children: Query<&Children>,
    name_query: Query<&Name>,
    visibility_query: Query<&Visibility>,
) {
    println!("-----------");

    for root in roots.iter() {
        print_entity_tree(root,
             &children, 
             &name_query,
             &visibility_query, 
             0);
    }
}

fn print_entity_tree(
    entity: Entity,
    children_query: &Query<&Children>,
    name_query: &Query<&Name>,
    visibility_query: &Query<&Visibility>,
    depth: usize,
) {
    let indent = "  ".repeat(depth);

    let mut line = format!("{}{} ", indent, entity);

    // If Name component exists, print it
    if let Ok(name) = name_query.get(entity) {
        line.push_str(&format!("({}) ", name.as_str()));
    }

    // Print visibility
    if let Ok(vis) = visibility_query.get(entity) {
        line.push_str(&format!("[{:?}]", vis));
    }

    println!("{}", line);

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            print_entity_tree(
                child, 
                children_query, 
                name_query,
                visibility_query,
                depth + 1);
        }
    }
}