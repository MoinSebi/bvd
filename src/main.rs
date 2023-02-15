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
use std::sync::Arc;
use gfaR_wrapper::{NGfa};
use log::{ info, warn};
use crate::bifurcation_algo::{bifurcation_bubble, bifurcation_bubble_lowmem};
use crate::graph_helper::{graph2pos, index_faster};
use crate::helper::chunk_by_index;
use crate::logging::newbuilder;
use crate::writer::write_wrapper;


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
            .about("Adds NL-tag (nestedness-level) to the stats output file - only working when --structure is provided [default: off]"))

        .help_heading("Comnputation ressources")
        .arg(Arg::new("threads")
            .short('t')
            .long("threads")
            .about("Number of threads")
            .default_value("1"))
        .arg(Arg::new("low-memory")
            .long("low-memory")
            .about("Run in low-memory option (slower is most cases)"))

        .help_heading("Processing information")
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
    // Reading number of threads
    let threads: usize = matches.value_of("threads").unwrap().parse().unwrap();

    // Reading the graph
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


    // Read the graph - using this function if faster because if ignores all edges (not needed here)
    info!("Reading the graph");
    let mut graph: NGfa = NGfa::new();
    graph.from_file_direct2(graph_file);
    let graph_f = Arc::new(graph);
    // For each path create a hashmap which returns for each index the exact position in the path

    // CLone the paths and path2index (not sure)


    // Bifurcation functions
    info!("Index graph");
    let mut intervals: Vec<(usize, u32, u32, u32)> = Vec::new();
    let mut bubbles: Vec<(u32, u32)> = vec![];
    if matches.is_present("low-memory"){
        info!("Running in low mem mode");
        //(intervals, bubbles) = bifurcation_bubble_lowmem(&graph, &threads);
    } else {
        let node_path_index = index_faster(&graph_f.paths, &threads);
        (intervals, bubbles) = bifurcation_bubble(&graph_f, &threads, node_path_index);
    }
    // Lets write bubble and other file at the same time
    info!("Number of intervals {}", intervals.len().clone());
    info!("Number of bubbles {}", bubbles.len());
    let chunks =  chunk_by_index(intervals, bubbles.len().clone() as u32, threads as u32);info!("Statistics and writing output");
    //
    let g2p = graph2pos(&graph_f);

    //
    // Write output
    write_wrapper(chunks, g2p, &graph_f.paths, out_prefix, bubbles);
    //
    // info!("Done");

    // TODO
    // - Add relationsship
    // - Add nestedness

}

