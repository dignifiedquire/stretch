pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node000: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(50f32),
                    height: stretch::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node001: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(50f32),
                    height: stretch::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node00: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                justify_content: stretch::style::JustifyContent::Center,
                size: stretch::geometry::Size { width: stretch::style::Dimension::Percent(1f32), ..Default::default() },
                ..Default::default()
            },
            vec![node000, node001],
        )
        .unwrap();
    let node0: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style { flex_direction: stretch::style::FlexDirection::Column, ..Default::default() },
            vec![node00],
        )
        .unwrap();
    let node: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                align_items: stretch::style::AlignItems::Center,
                justify_content: stretch::style::JustifyContent::Center,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(200f32),
                    height: stretch::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0],
        )
        .unwrap();
    stretch.compute_layout(&node, stretch::geometry::Size::undefined()).unwrap();
}
