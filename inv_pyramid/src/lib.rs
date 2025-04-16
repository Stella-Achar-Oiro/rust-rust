pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut pyramid = Vec::new();
    for j in 1..i{
        pyramid.push(format!("{}{}", " ".repeat(j as usize), v.repeat(j as usize)));
    }
    for j in (1..=i).rev() {
        pyramid.push(format!("{}{}", " ".repeat(j as usize), v.repeat(j as usize)));
    }
    pyramid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inv_pyramid() {
        let data_sets = vec![
            vec![],
            vec![" #"],
            vec![" a", "  aa", " a"],
            vec![
                " >",
                "  >>",
                "   >>>",
                "    >>>>",
                "     >>>>>",
                "    >>>>",
                "   >>>",
                "  >>",
                " >",
            ],
            vec![
                " &",
                "  &&",
                "   &&&",
                "    &&&&",
                "     &&&&&",
                "      &&&&&&",
                "       &&&&&&&",
                "        &&&&&&&&",
                "       &&&&&&&",
                "      &&&&&&",
                "     &&&&&",
                "    &&&&",
                "   &&&",
                "  &&",
                " &",
            ],
        ];
        assert_eq!(inv_pyramid(String::from("#"), 0), data_sets[0]);
        assert_eq!(inv_pyramid(String::from("#"), 1), data_sets[1]);
        assert_eq!(inv_pyramid(String::from("a"), 2), data_sets[2]);
        assert_eq!(inv_pyramid(String::from(">"), 5), data_sets[3]);
        assert_eq!(inv_pyramid(String::from("&"), 8), data_sets[4]);
    }
}
#[test]
fn it_works() {
    let data_sets = vec![
        vec![],
        vec![" #"],
        vec![" a", "  aa", " a"],
        vec![
            " >",
            "  >>",
            "   >>>",
            "    >>>>",
            "     >>>>>",
            "    >>>>",
            "   >>>",
            "  >>",
            " >",
        ],
        vec![
            " &",
            "  &&",
            "   &&&",
            "    &&&&",
            "     &&&&&",
            "      &&&&&&",
            "       &&&&&&&",
            "        &&&&&&&&",
            "       &&&&&&&",
            "      &&&&&&",
            "     &&&&&",
            "    &&&&",
            "   &&&",
            "  &&",
            " &",
        ],
    ];
    assert_eq!(inv_pyramid(String::from("#"), 0), data_sets[0]);
    assert_eq!(inv_pyramid(String::from("#"), 1), data_sets[1]);
    assert_eq!(inv_pyramid(String::from("a"), 2), data_sets[2]);
    assert_eq!(inv_pyramid(String::from(">"), 5), data_sets[3]);
    assert_eq!(inv_pyramid(String::from("&"), 8), data_sets[4]);
}