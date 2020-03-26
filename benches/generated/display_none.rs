pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node0: stretch::node::StretchNode =
        stretch.new_node(stretch::style::Style { flex_grow: 1f32, ..Default::default() }, vec![]).unwrap();
    let node1: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style { display: stretch::style::Display::None, flex_grow: 1f32, ..Default::default() },
            vec![],
        )
        .unwrap();
    let node: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0, node1],
        )
        .unwrap();
    stretch.compute_layout(&node, stretch::geometry::Size::undefined()).unwrap();
}
