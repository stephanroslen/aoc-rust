use rustc_hash::{FxHashMap, FxHashSet};

fn bron_kerbosch(
    links: &FxHashMap<[u8; 2], FxHashSet<[u8; 2]>>,
    r: FxHashSet<[u8; 2]>,
    mut p: FxHashSet<[u8; 2]>,
    mut x: FxHashSet<[u8; 2]>,
) -> Option<FxHashSet<[u8; 2]>> {
    if p.is_empty() && x.is_empty() {
        return Some(r);
    }

    let vs = p.iter().copied().collect::<Box<[_]>>();

    let mut candidate: Option<FxHashSet<[u8; 2]>> = None;

    for v in vs {
        let nv = links.get(&v).expect("Neighbours expected");
        let mut ruv = r.clone();
        ruv.insert(v);
        let pinv = p.intersection(nv).cloned().collect::<FxHashSet<_>>();
        let xinv = x.intersection(nv).cloned().collect::<FxHashSet<_>>();
        let result = bron_kerbosch(links, ruv, pinv, xinv);

        if result.is_some()
            && (candidate.is_none()
                || result.as_ref().expect("Result expected").len()
                    > candidate.as_ref().expect("Candidate expected").len())
        {
            candidate = result;
        }

        p.remove(&v);
        x.insert(v);
    }

    candidate
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_input = std::fs::read_to_string("input")?;
    let raw_lines = raw_input.trim_end().split('\n').collect::<Box<[_]>>();

    let links = {
        let mut links = FxHashMap::<[u8; 2], FxHashSet<[u8; 2]>>::default();

        for (from, to) in raw_lines
            .iter()
            .map(|s| s.split_once('-').expect("Dash expected"))
        {
            let from = from.as_bytes();
            let from = [from[0], from[1]];
            let to = to.as_bytes();
            let to = [to[0], to[1]];
            links.entry(from).or_default().insert(to);
            links.entry(to).or_default().insert(from);
        }

        links
    };

    let result1 = {
        let mut cliques_of_three = FxHashSet::default();

        for (&peer0, links_to) in &links {
            for &peer1 in links_to {
                if peer1 <= peer0 {
                    continue;
                }
                for &peer2 in links_to {
                    if peer2 <= peer1 {
                        continue;
                    }
                    if links
                        .get(&peer1)
                        .expect("Links for node expected")
                        .contains(&peer2)
                    {
                        let clique = [peer0, peer1, peer2];

                        cliques_of_three.insert(clique);
                    }
                }
            }
        }

        cliques_of_three
            .iter()
            .filter(|c| {
                c.iter().any(|name| {
                    std::str::from_utf8(name)
                        .expect("Valid utf-8 expected")
                        .starts_with('t')
                })
            })
            .count()
    };

    println!("{}", result1);

    let result2 = {
        let vertices = links.keys().copied().collect::<FxHashSet<_>>();
        let biggest_clique =
            bron_kerbosch(&links, FxHashSet::default(), vertices, FxHashSet::default())
                .expect("Biggest clique expected");

        let mut biggest_clique_collected = biggest_clique
            .iter()
            .map(|c| std::str::from_utf8(c).expect("Valid utf-8 expected"))
            .collect::<Box<[_]>>();
        biggest_clique_collected.sort();
        biggest_clique_collected.join(",")
    };

    println!("{}", result2);

    Ok(())
}
