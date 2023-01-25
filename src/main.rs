#[allow(non_snake_case)]
pub mod bifurcation_algo;
mod helper;
mod logging;
mod writer;
mod graph_helper;
mod bifurcation_helper;

use clap::{Arg, App, AppSettings};
use std::path::Path;
use std::process;
use gfaR_wrapper::{NGfa};
use log::{ info, warn};
use crate::bifurcation_algo::{bifurcation_bubble, bifurcation_bubble_lowmem, bvd_low_memory};
use crate::graph_helper::{graph2pos, index_faster};
use crate::helper::chunk_by_index;
use crate::logging::newbuilder;
use crate::writer::{write_wrapper};


fn main() {
    let matches = App::new("bvd")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.0")
        .author("Sebastian V")
        .about("Bifurcation variation detection")

        .help_heading("Input options")
        .arg(Arg::new("gfa")
            .short('g')
            .long("gfa")
            .about("Input GFA file")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("delimiter")
        .short('d')
        .long("delimiter")
        .about("Delimiter for between genome and chromosome")
        .takes_value(true))


        .help_heading("Output options")
        .arg(Arg::new("output")
            .display_order(1)
            .short('o')
            .long("output")
            .about("Output prefix")
            .takes_value(true)
            .default_value("panSV.output"))
        .arg(Arg::new("Structure")
            .long("structure")
            .short('s')
            .about("Report the structure/relationship of the bubbles to each other"))
        .arg(Arg::new("Nestedness")
            .long("nestedness")
            .about("Adds NL-tag (nestedness-level) to the stats output file [default: off]"))
        .help_heading("Threading")
        .arg(Arg::new("threads")
            .short('t')
            .long("threads")
            .about("Number of threads")
            .default_value("1"))
        .help_heading("Processing information")
        .arg(Arg::new("low-memory")
            .long("low-memory")
            .about("Run in low-memory option (slower is most cases)"))
        .arg(Arg::new("quiet")
            .short('q')
            .about("No updating INFO messages"))
        .arg(Arg::new("verbose")
            .short('v')
            .about("-v = DEBUG | -vv = TRACE")
            .takes_value(true)
            .default_missing_value("v1"))
        .get_matches();

    // Checking verbose
    // Ugly, but needed - May end up in a small library later
    newbuilder(&matches);

    //-------------------------------------------------------------------------------------------------

    info!("Running bvd");
    let threads: usize = matches.value_of("threads").unwrap().parse().unwrap();

    // Check if graph is running
    let mut graph_file = "not_relevant";
    if matches.is_present("gfa") {
        if Path::new(matches.value_of("gfa").unwrap()).exists() {
            graph_file = matches.value_of("gfa").unwrap();
        } else {
            warn!("No file with such name");
            process::exit(0x0100);
        }

    }

    // This is the prefix
    let out_prefix= matches.value_of("output").unwrap();


    // Read the graph
    info!("Reading the graph");
    let mut graph: NGfa = NGfa::new();
    // This is smaller (+faster?), because it does not read the edges and no sequence.
    graph.from_file_direct2(graph_file);

    // Counting nodes
    let g2p = graph2pos(&graph);



    let paths = graph.paths.clone();
    let _p2id = graph.path2id.clone();


    // Bifurcation stuff
    info!("Index graph");
    let intervals: Vec<(usize, u32, u32, u32)>;
    let bubbles: Vec<(u32, u32)>;
    if matches.is_present("low-memory"){
        info!("Running in low mem modus");
        let intermed = bifurcation_bubble_lowmem(&graph, &threads);
        intervals = intermed.0;
        bubbles = intermed.1;
    } else {
        let (node_hashset, node_path_index) = index_faster(&graph.paths, &threads);
        let intermed = bifurcation_bubble(&graph, &threads, node_hashset, node_path_index);
        intervals = intermed.0;
        bubbles = intermed.1;

    }
    // Lets write bubble and other file at the same time
    info!("Number of intervals {}", intervals.len());
    info!("Number of bubbles {}", bubbles.len());
    let chunks =  chunk_by_index(intervals, bubbles.len() as u32, threads as u32);info!("Statistics and writing output");

    // Write output
    write_wrapper(chunks, g2p, paths, out_prefix);

    info!("Done");

}

