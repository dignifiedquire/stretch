pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    stretch.compute_layout(&node, stretch::geometry::Size::undefined()).unwrap();
}
