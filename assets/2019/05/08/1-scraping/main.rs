use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct Details {
    name: String,
    types: Vec<String>,
    species: String,
    height: String,
    weight: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get("https://pokemondb.net/pokedex/national")?;

    for node in Document::from_read(res).unwrap().find(Class("ent-name")) {
        // node confirmed has href
        let link = node.attr("href").unwrap();

        let res = reqwest::get(&format!("https://pokemondb.net{}", link))?;
        let doc = Document::from_read(res).unwrap();

        // let res = include_str!("bulbasaur");
        // let doc = Document::from(res);

        // find name (v1)
        // let mut name = String::new();
        // for node in doc.find(Name("h1")) {
        //     name = node.text();
        // }

        // find name (v2)
        let name = doc.find(Name("h1")).next().unwrap().text();

        // println!("{:?}", name);

        // find types (v1)
        // let mut types = Vec::new();
        // for t in doc.find(Class("vitals-table").descendant(Class("type-icon"))) {
        //     types.push(t.text());
        // }

        // find types (v2)
        let types = doc
            .find(Class("vitals-table").descendant(Class("type-icon")))
            .map(|n| n.text())
            .collect::<Vec<_>>();

        // println!("{:?}", types);

        // find species (v2)
        // let nodes = doc.find(Class("vitals-table").descendant(Name("td")));
        // for node in nodes {
        //     println!("{:?}", node);
        // }

        // other details (v2)
        // let mut nodes = doc.find(Class("vitals-table").descendant(Name("td")));
        // nodes.next();
        // nodes.next();
        // let species = nodes.next().unwrap().text();
        // let height = nodes.next().unwrap().text();
        // let weight = nodes.next().unwrap().text();

        // other details (v3)
        let mut nodes = doc
            .find(Class("vitals-table").descendant(Name("td")))
            .skip(2);
        let species = nodes.next().unwrap().text();
        let height = nodes.next().unwrap().text();
        let weight = nodes.next().unwrap().text();

        // serialization (v1)
        // println!(
        //     r#"{{"name":"{}","types":{:?},"species":"{}","height":"{}","weight":"{}"}}"#,
        //     name, types, species, height, weight
        // );

        // serialization (v2)
        // let details = json!({
        //     "name": name,
        //     "types": types,
        //     "species": species,
        //     "height": height,
        //     "weight": weight
        // });
        // println!("{}", details.to_string());

        // serialization (v3)
        let details = Details {
            name,
            types,
            species,
            height,
            weight,
        };
        println!("{}", serde_json::to_string(&details)?);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // functional style
    // Document::from_read(res)?
    //     .find(Class("ent-name"))
    //     .filter_map(|node| node.attr("href"))
    //     .map(|link| reqwest::get(&format!("https://pokemondb.net{}", link)).unwrap())
    //     .map(|html| Document::from_read(html).unwrap())
    //     .for_each(|doc| {
    //         let name = doc.find(Name("h1")).next().unwrap().text();

    //         let types = doc
    //             .find(Class("vitals-table").descendant(Class("type-icon")))
    //             .map(|n| n.text())
    //             .collect::<Vec<_>>();

    //         let mut nodes = doc
    //             .find(Class("vitals-table").descendant(Name("td")))
    //             .skip(2);
    //         let species = nodes.next().unwrap().text();
    //         let height = nodes.next().unwrap().text();
    //         let weight = nodes.next().unwrap().text();

    //         let details = Details {
    //             name,
    //             types,
    //             species,
    //             height,
    //             weight,
    //         };
    //         println!("{}", serde_json::to_string(&details).unwrap());

    //         std::thread::sleep(std::time::Duration::from_secs(1));
    //     });

    Ok(())
}
