use crate::startin::Triangulation;

use startin;

fn five_points() -> Triangulation {
    let mut pts: Vec<[f64; 3]> = Vec::new();
    pts.push([0.0, 0.0, 1.0]);
    pts.push([10.0, 0.0, 2.0]);
    pts.push([10.0, 10.0, 3.0]);
    pts.push([0.0, 10.0, 4.0]);
    pts.push([5.0, 5.0, 10.0]);
    let mut dt = startin::Triangulation::new();
    dt.insert(&pts, startin::InsertionStrategy::AsIs);
    dt
}

#[test]
fn duplicate_alltests() {
    let mut dt = five_points();

    dt.set_duplicates_handling(startin::DuplicateHandling::First);
    let re = dt.insert_one_pt(5.0, 5.0, 20.0);
    assert_eq!(Err((5, false)), re);
    match re {
        Ok(_) => (),
        Err((i, _b)) => assert_eq!(dt.get_point(i).unwrap()[2], 10.0),
    }

    dt.set_duplicates_handling(startin::DuplicateHandling::Last);
    let re = dt.insert_one_pt(5.0, 5.0, 20.0);
    assert_eq!(Err((5, true)), re);
    match re {
        Ok(_) => (),
        Err((i, _b)) => assert_eq!(dt.get_point(i).unwrap()[2], 20.0),
    }

    dt.set_duplicates_handling(startin::DuplicateHandling::Highest);
    let re = dt.insert_one_pt(5.0, 5.0, 21.0);
    assert_eq!(Err((5, true)), re);
    match re {
        Ok(_) => (),
        Err((i, _b)) => assert_eq!(dt.get_point(i).unwrap()[2], 21.0),
    }

    dt.set_duplicates_handling(startin::DuplicateHandling::Lowest);
    let re = dt.insert_one_pt(5.0, 5.0, 5.0);
    assert_eq!(Err((5, true)), re);
    match re {
        Ok(_) => (),
        Err((i, _b)) => assert_eq!(dt.get_point(i).unwrap()[2], 5.0),
    }
}
