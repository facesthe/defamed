//! Permutation generation for positional and named parameters.
#![allow(unused)]

pub mod fields;
pub mod params;

/// Data from the `#[def]` attribute
#[derive(Clone)]
pub enum ParamAttr {
    /// No helper attribute
    None,
    // Use default trait for initialization
    Default,
    // Use const expr for initialization
    Value(syn::Expr),
}

/// A single permuted item
#[derive(Clone)]
enum PermutedItem<T: Clone> {
    /// Item is positional with no identifier
    Positional(T),
    /// Item is named with an identifier
    Named(T),
    /// Item is a default value
    Default(T),
}

impl<T: Clone> PermutedItem<T> {
    /// Get the inner value
    fn inner(&self) -> &T {
        match self {
            Self::Positional(i) => i,
            Self::Named(i) => i,
            Self::Default(i) => i,
        }
    }

    /// Split the items into used and unused default values while maintaining order.
    /// The items must only contain named and default variants.
    fn parition_named_defaults(
        items: &[PermutedItem<T>],
    ) -> (Vec<PermutedItem<T>>, Vec<PermutedItem<T>>) {
        let res: (Vec<_>, Vec<_>) = items.iter().cloned().partition(|def| match def {
            PermutedItem::Named(_) => true,
            PermutedItem::Default(_) => false,
            _ => panic!("unexpected variant"),
        });

        res
    }
}

/// Generate all permutations of positional items and default items.
///
/// Returns a matrix of tuples of positional and default permutations.
fn permute<T: Clone>(
    required: Vec<T>,
    default: Vec<T>,
) -> Vec<(Vec<PermutedItem<T>>, Vec<PermutedItem<T>>)> {
    let named_permute = (0..=required.len())
        .flat_map(|idx| {
            // let opp_idx = required.len() - i;
            let (positional, named) = required.split_at(idx);

            let positional = positional
                .iter()
                .map(|p| PermutedItem::Positional(p.to_owned()))
                .collect::<Vec<_>>();
            let permute_slice = permute_named(named);

            permute_slice
                .iter()
                .map(|named_seq| [positional.as_slice(), named_seq.as_slice()].concat())
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect::<Vec<_>>();

    let default_permute = permute_named_default(&default);
    let default_positional_permute = permute_pos_default(&default);

    // last element in named permutation matrix contains all positional parameters
    let all_positional = match named_permute.last() {
        Some(base) => {
            // sanity check
            assert!(
                base.iter()
                    .all(|item| matches!(item, PermutedItem::Positional(_))),
                "all permuted items in the last permuted sequence must be positional."
            );

            default_positional_permute
                .into_iter()
                .map(|seq| (base.to_vec(), seq.to_vec()))
                .collect::<Vec<_>>()
        }
        None => default_positional_permute
            .into_iter()
            .map(|seq| (vec![], seq))
            .collect(),
    };

    let named_pos = match (named_permute.len(), default_permute.len()) {
        // we do not append completely empty sequences
        (0, 0) => return all_positional,
        (0, _) => default_permute
            .into_iter()
            .map(|seq| (vec![], seq))
            .collect::<Vec<_>>(),
        (_, 0) => named_permute
            .into_iter()
            .map(|seq| (seq, vec![]))
            .collect::<Vec<_>>(),
        _ => named_permute
            .into_iter()
            .flat_map(|seq| {
                default_permute
                    .iter()
                    .map(move |def_seq| (seq.to_vec(), def_seq.to_vec()))
            })
            .collect::<Vec<_>>(),
    };

    [named_pos, all_positional].concat()
}

/// Perform permutations of all items in a slice.
/// All items will be wrapped in [PermutedItem::Named].
fn permute_named<T: Clone>(named: &[T]) -> Vec<Vec<PermutedItem<T>>> {
    let permutations = permute::permutations_of(named);

    permutations
        .into_iter()
        .map(|single_perm| {
            single_perm
                .into_iter()
                .map(|item| PermutedItem::Named(item.clone()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

/// Perform permutations for default parameters. All permuted values are named.
/// For permutations of positional defaults see [permute_pos_default].
///
/// Each item in the slice must have a default value.
/// This function will not check for this.
///
/// Additionally, default params can be used(named) or unused(default). These are also permuted as well.
fn permute_named_default<T: Clone>(defaults: &[T]) -> Vec<Vec<PermutedItem<T>>> {
    let base_permute = (0..(1 << defaults.len()))
        .map(|num| {
            let seq = defaults
                .iter()
                .enumerate()
                .map(|(pos, item)| {
                    // if bit set, it is used
                    if (num >> pos) & 1 != 0 {
                        PermutedItem::Named(item.to_owned())
                    } else {
                        PermutedItem::Default(item.to_owned())
                    }
                })
                .collect::<Vec<_>>();

            seq
        })
        .collect::<Vec<_>>();

    let res = base_permute
        .into_iter()
        .flat_map(|seq| {
            let (used, unused) = PermutedItem::<T>::parition_named_defaults(&seq);

            let mut used_permute = permute::permute(used);

            for item in &mut used_permute {
                item.extend_from_slice(&unused);
            }

            used_permute.into_iter()
        })
        .collect::<Vec<_>>();

    res.into_iter().filter(|item| !item.is_empty()).collect()
}

/// Perform permutations for positional default permutations.
///
/// This is for the special case where all preceding non-default parameters
/// are used as positional parameters.
fn permute_pos_default<T: Clone>(defaults: &[T]) -> Vec<Vec<PermutedItem<T>>> {
    let res = (1..=defaults.len())
        .flat_map(|idx| {
            let (positional, named) = defaults.split_at(idx);
            let pos_params = positional
                .iter()
                .map(|p| PermutedItem::Positional(p.to_owned()))
                .collect::<Vec<_>>();

            let inter = match named.len() {
                0 => vec![pos_params],
                _ => {
                    let named_permute = permute_named_default(named);

                    named_permute
                        .into_iter()
                        .map(move |named_seq| [pos_params.clone(), named_seq].concat())
                        .collect::<Vec<_>>()
                }
            };

            inter.into_iter()
        })
        .collect::<Vec<_>>();

    res
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Test only named parameters
    #[test]
    fn test_permute_named() {
        let items = vec!["a", "b", "c", "d"];

        let permutations = permute_named(&items);
        // println!("{:?}", permutations);
        assert_eq!(permutations.len(), 24);

        let _ = permutations
            .into_iter()
            .flatten()
            .into_iter()
            .map(|item| {
                assert!(matches!(item, PermutedItem::Named(_)));
            })
            .collect::<Vec<_>>();
    }

    /// Test only named defaults
    #[test]
    fn test_permute_named_defaults() {
        let mut items = vec!["a", "b"];

        let permutations = permute_named_default(&items);

        // 0 0
        // 0 1
        // 1 0
        // 1 1
        // 1 1 again because used defaults have to be permuted
        assert_eq!(permutations.len(), 5);

        items.clear();
        let permutations = permute_pos_default(&items);
        assert!(permutations.is_empty());
    }

    /// Test only positional defaults
    #[test]
    fn test_permute_positional_defaults() {
        let items = vec!["a", "b", "c"];

        let permutations = permute_pos_default(&items);

        // 0 0
        // 0 1
        // 1 0
        // 1 1
        assert_eq!(permutations.len(), 8);
    }

    /// Full permutation test without any default parameters
    #[test]
    fn test_permute_all_positional_named() {
        let items = vec!["a", "b", "c", "d"];

        let permutations = permute(items, vec![]);

        assert_eq!(permutations.len(), 34);
    }

    /// Test everything with default parameters
    #[test]
    fn test_permute_all_positional_default() {
        let items = vec!["a", "b", "c", "d"];
        let defaults = vec!["e", "f"];

        let permutations = permute(items, defaults);

        // 5 permutations for default parameters
        // and 3 additional permutations for positional default parameters (not permuted)
        assert_eq!(permutations.len(), 34 * 5 + 3);
    }
}
