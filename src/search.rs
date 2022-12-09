use std::iter;

use cauldron_data::{item::ItemDescriptor, Data};

pub fn search_enumerate<'a: 'b, 'b, T, I>(
    query: &str,
    items: &'a [T],
    f: impl Fn(&'b T) -> I,
) -> impl ExactSizeIterator<Item = (usize, &'a T)>
where
    T: 'b,
    I: IntoIterator<Item = (isize, &'b str)> + 'b,
{
    let mut results = Vec::with_capacity(items.len());

    for (i, item) in items.iter().enumerate() {
        for (offset, string) in f(item) {
            let Some(best) = sublime_fuzzy::best_match(query, string) else {
                continue;
            };

            results.push((best.score() + offset, i, item));
            break;
        }
    }

    results.sort_by(|a, b| a.0.cmp(&b.0).reverse());
    results.into_iter().map(|(_, i, item)| (i, item))
}

pub fn search_item_descriptors<'a>(
    query: &str,
    data: &'a Data,
) -> impl ExactSizeIterator<Item = (usize, &'a ItemDescriptor)> {
    search_enumerate(query, &data.items, |item| {
        let name = iter::once((0, item.name.as_str()));
        let id = item.id.iter().map(|id| (-1000, id.as_str()));
        let categories = item.categories.iter().map(|s| (-10000, s.as_str()));
        let description = iter::once((-100000, item.description.as_str()));

        name.chain(id).chain(categories).chain(description)
    })
}
