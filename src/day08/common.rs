use core;

pub type Error = core::Error;

pub struct Node {
    pub children: Vec<Node>,
    pub metadata: Vec<u32>,
}

pub type Graph = Node;

pub fn get_data(session: &str) -> Result<Graph, core::Error> {
    let content = try!(core::request(session, 8));

    parse_values(content.trim().split(" "))
}

pub fn parse_values<'a, Iin>(values: Iin) -> Result<Graph, core::Error>
where
    Iin: Iterator<Item = &'a str>,
{
    start_parsing(values.collect::<Vec<&str>>().as_slice()).map(|(graph, _)| graph)
}

fn start_parsing<'a>(values: &[&str]) -> Result<(Graph, u32), core::Error>
{
    if values.len() < 2 {
        return Err(core::Error::Internal{why: "missing header".to_string()})
    }
    let (children_n, metadata_n) = (try!(values[0].parse::<u32>()), try!(values[1].parse::<u32>()));
    let mut children_offset = 2;
    Ok((Graph{
        children: try!((0..children_n).map(|_| {
            let (child, offset) = try!(start_parsing(&values[children_offset..]));
            children_offset += offset as usize;
            Ok(child)
        }).collect::<Result<Vec<Graph>, core::Error>>()),
        metadata: try!((0..metadata_n).map(|i| {
            Ok(try!(values[children_offset + i as usize].parse::<u32>()))
        }).collect::<Result<Vec<u32>, core::Error>>()),
    }, children_offset as u32 + metadata_n))

}
