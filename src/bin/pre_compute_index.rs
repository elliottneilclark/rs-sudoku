use rs_sudoku::slow_index::*;

type IterFn = dyn Fn(usize) -> RelatedIndexIterator;
fn main() {
    let fields: [(&str, Box<IterFn>); 3] = [
        (
            "ROW_DATA",
            Box::new(|idx| RelatedIndexIterator {
                incr: 0,
                gp: Box::new(RowGenPosition::new(idx)),
            }),
        ),
        (
            "COLUMN_DATA",
            Box::new(|idx| RelatedIndexIterator {
                incr: 0,
                gp: Box::new(ColumnGenPosition::new(idx)),
            }),
        ),
        (
            "BOX_DATA",
            Box::new(|idx| RelatedIndexIterator {
                incr: 0,
                gp: Box::new(BoxGenPosition::new(idx)),
            }),
        ),
    ];

    for (name, g) in fields.iter() {
        let d = (0..9)
            .map(|idx| {
                g(idx)
                    .map(|u| format!("{:3 }", u))
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .collect::<Vec<String>>();
        println!(
            "pub const {} : [u8; 81] = [\n        {}];",
            name,
            d.join(",\n        ")
        );
    }
}
