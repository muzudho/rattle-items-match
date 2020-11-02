extern crate rattle_items_match;

use rattle_items_match::{
    ActualBuilder, AnyBuilder, Controls as Co, Element as El, ExpectedBuilder, MachineBuilder,
    Quantity as Qu, RangeContainsMaxBuilder, RepeatBuilder,
};

fn main() {
    println!("Start.");

    let act1_ssss1 = ActualBuilder::default()
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let act2_tsss1 = ActualBuilder::default()
        .push(&'\t')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let act3_xsss1 = ActualBuilder::default()
        .push(&'x')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();
    let act4_a = ActualBuilder::default().push(&'A').build();
    let act5_bc = ActualBuilder::default().push(&'B').push(&'C').build();
    let act6_de = ActualBuilder::default().push(&'d').push(&'e').build();
    let act7_fgh = ActualBuilder::default()
        .push(&'f')
        .push(&'g')
        .push(&'h')
        .build();

    // Whitespace characters.
    let wschar = AnyBuilder::default()
        .push(&El::Exact('\t'))
        .push(&El::Exact(' '))
        .build();

    // Digit.
    let digit = RangeContainsMaxBuilder::default()
        .set_min(&'0')
        .set_max(&'9')
        .build();
    // Alphabet.
    let upper_case = El::RangeIncludesMax(
        RangeContainsMaxBuilder::default()
            .set_min(&'A')
            .set_max(&'Z')
            .build(),
    );
    let lower_case = El::RangeIncludesMax(
        RangeContainsMaxBuilder::default()
            .set_min(&'a')
            .set_max(&'z')
            .build(),
    );
    let alpha = AnyBuilder::default()
        .push(&upper_case)
        .push(&lower_case)
        .build();

    let expected1_wsss1 = ExpectedBuilder::default()
        .push(&Co::Once(Qu::Any(wschar.clone())))
        .push(&Co::Once(Qu::One(El::Exact(' '))))
        .push(&Co::Once(Qu::One(El::Exact(' '))))
        .push(&Co::Once(Qu::One(El::Exact(' '))))
        .push(&Co::Once(Qu::One(El::Exact('1'))))
        .build();

    let expected2_ws1max = ExpectedBuilder::default()
        .push(&Co::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Qu::Any(wschar.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::Exact('1'))))
        .build();
    let expected3_ws5max = ExpectedBuilder::default()
        .push(&Co::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Qu::Any(wschar.clone()))
                .set_min(5)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::Exact('1'))))
        .build();
    let expected4_ws03 = ExpectedBuilder::default()
        .push(&Co::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Qu::Any(wschar.clone()))
                .set_min(0)
                .set_max_not_included(3)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::Exact('1'))))
        .build();
    let expected5_ws1max = ExpectedBuilder::default()
        .push(&Co::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Qu::Any(wschar.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::RangeIncludesMax(digit))))
        .build();
    let expected6_alpha = ExpectedBuilder::default()
        .push(&Co::Once(Qu::Any(alpha.clone())))
        .build();
    let expected7_alpha1to3 = ExpectedBuilder::default()
        .push(&Co::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Qu::Any(alpha.clone()))
                .set_min(1)
                .set_max_not_included(3)
                .build(),
        ))
        .build();
    let expected8_alpha1to_max = ExpectedBuilder::default()
        .push(&Co::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Qu::Any(alpha.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .build();

    assert!(MachineBuilder::default()
        .set_actual_items(&act1_ssss1)
        .set_expected_items(&expected1_wsss1)
        .build()
        .matching());
    assert!(MachineBuilder::default()
        .set_actual_items(&act2_tsss1)
        .set_expected_items(&expected1_wsss1)
        .build()
        .matching());
    assert!(!MachineBuilder::default()
        .set_actual_items(&act3_xsss1)
        .set_expected_items(&expected1_wsss1)
        .build()
        .matching());
    assert!(MachineBuilder::default()
        .set_actual_items(&act1_ssss1)
        .set_expected_items(&expected2_ws1max)
        .build()
        .matching());
    assert!(!MachineBuilder::default()
        .set_actual_items(&act1_ssss1)
        .set_expected_items(&expected3_ws5max)
        .build()
        .matching());
    assert!(!MachineBuilder::default()
        .set_actual_items(&act1_ssss1)
        .set_expected_items(&expected4_ws03)
        .build()
        .matching());
    assert!(MachineBuilder::default()
        .set_actual_items(&act1_ssss1)
        .set_expected_items(&expected5_ws1max)
        .build()
        .matching());
    assert!(MachineBuilder::default()
        .set_actual_items(&act4_a)
        .set_expected_items(&expected6_alpha)
        .build()
        .matching());
    assert!(MachineBuilder::default()
        .set_actual_items(&act5_bc)
        .set_expected_items(&expected7_alpha1to3)
        .build()
        .matching());
    assert!(MachineBuilder::default()
        .set_actual_items(&act6_de)
        .set_expected_items(&expected7_alpha1to3)
        .build()
        .matching());
    assert!(MachineBuilder::default()
        .set_actual_items(&act7_fgh)
        .set_expected_items(&expected8_alpha1to_max)
        .build()
        .matching());
    println!("Finished.");
}
