
use clap::{Arg, App, AppSettings};
use std::path::Path;
use std::process;
use std::sync::Arc;
use gfaR_wrapper::{NGfa};
use log::{ info, warn};
use bvd::bifurcation_algo::{bifurcation_bubble, bifurcation_bubble_lowmem};
use bvd::graph_helper::{graph2pos, node2index_wrapper};
use bvd::helper::{chunk_by_index, chunk_by_index2, getSlice_test};
use bvd::logging::newbuilder;
use bvd::writer::{solo_stats, write_wrapper};


/// TODO:
/// - Update merge function
/// - Improve runtime on stats
/// - Check why bifurcation is so slow
/// - Adjust reader
/// - Remove the timing stuff
///
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
    let out_prefix = matches.value_of("output").unwrap();


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
    if matches.is_present("low-memory") {
        info!("Running in low mem mode");
        //(intervals, bubbles) = bifurcation_bubble_lowmem(&graph, &threads);
    } else {
        // Index node->index - more in the function comment
        let (node_index_sort, node_to_index) = node2index_wrapper(&graph_f.paths, &threads);
        (intervals, bubbles) = bifurcation_bubble(&graph_f, &threads, node_index_sort, node_to_index);
    }
    info!("slicer dicer");
    // intervals.sort();
    // let mut test = Vec::new();
    // for x in intervals.iter(){
    //     if x.0 != 0{
    //         break
    //     }
    //     test.push(x.clone());
    // }
    // info!("slicer dicer2");
    //
    // getSlice_test(&mut test, &graph_f.paths[0], &g2p[0]);
    // // Lets write bubble and other file at the same time
    // info!("Number of intervals {}", intervals.len().clone());
    // info!("Number of bubbles {}", bubbles.len());
    //let chunks2 = chunk_by_index2(intervals.clone(), bubbles.len().clone() as u32, threads as u32);

    //let chunks = chunk_by_index(&mut intervals, bubbles.len().clone() as u32, threads as u32);
    info!("Statistics and writing output");
    //
    //let g2p = graph2pos(&graph_f);

    //
    // Write output
    let g2p = graph2pos(&graph_f);
    solo_stats(&intervals[..],g2p, graph_f.paths.clone(), bubbles.clone());
    //let f = graph_f.paths.clone();
    //drop(graph_f);
    //write_wrapper(chunks2, g2p, f, out_prefix, bubbles);
    //
    info!("Done");



}

