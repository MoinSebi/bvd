use std::path::Path;
use std::process;
//
use clap::{Arg, App, AppSettings};
use gfa_reader::{GraphWrapper, NCGfa, NCPath};
use hashbrown::HashSet;
// use std::path::Path;
// use std::process;
// use std::sync::Arc;
// use gfa_reader::{GraphWrapper, NCGfa, NCPath};
// use gfaR_wrapper::{NGfa};
use log::{ info, warn};
use related_intervals::{make_nested, sort_vector};
use bvd::bifurcation_algo::{bvd_wrapper, get_interval};
use bvd::helper::chunk_by_index2;
//use bvd::bifurcation_algo::bifurcation_bubble_lowmem;
// use bvd::bifurcation_algo::{bifurcation_bubble, bifurcation_bubble_lowmem};
// use bvd::graph_helper::{graph2pos, node2index_wrapper};
// use bvd::helper::{chunk_by_index, chunk_by_index2, getSlice_test};
use bvd::logging::newbuilder;
use bvd::relation::get_relations;
use bvd::writer::{all_one, stats, tdsatda, write_bubbles};
// use bvd::writer::{solo_stats, write_wrapper};


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
            .about("Input GFA file (v1)")
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

    // Get graph name
    let mut graph_file = "not_relevant";
    if matches.is_present("gfa") {
        if Path::new(matches.value_of("gfa").unwrap()).exists() {
            graph_file = matches.value_of("gfa").unwrap();
        } else {
            warn!("No file with such name");
            process::exit(0x0100);
        }
    }

    // Get output prefix
    let out_prefix = matches.value_of("output").unwrap();


    // Read the graph - using this function if faster because if ignores all edges (not needed here)
    // Also add the graph wrapper
    info!("Reading the graph");
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_direct(graph_file, false);
    let mut wrapper: GraphWrapper<NCPath> = GraphWrapper::new();
    wrapper.from_gfa(&graph.paths, " ");
    // For each path create a hashmap which returns for each index the exact position in the path

    // CLone the paths and path2index (not sure)


    // Bifurcation functions

    // You recalculate the node2index every time
    if matches.is_present("low-memory") {
        info!("Running in low mem mode");

    }

    let (mut interval, bubbles) = get_interval(&graph, &threads);


    write_bubbles(&bubbles, out_prefix);
    info!("BVD: Total number of bubbles: {}", bubbles.len());
    info!("BVD: Total number of intervals: {}", interval.len());

    //a.sort();

    info!("BVD: Identify bubbles structures");
    let mut c = get_relations(&interval, &threads);


    info!("BVD: Number of relations {}", c.len());

    // Flat out the data
    let mut flattened_with_index: Vec<(usize, u32, u32, u32)> = interval
        .iter()
        .enumerate()
        .flat_map(|(index, inner_vec)| {
            inner_vec.1
                .iter()
                .map(move |&a| (index, a[0], a[1], a[2]))
        })
        .collect();

    flattened_with_index.sort_by_key(|a| (a.3));
    //println!("{:?}", flattened_with_index);
    let max_bubble = flattened_with_index.last().unwrap().3;

    info!("pre");

    all_one(&mut flattened_with_index, &graph);

    info!("pre1");
    //
    // let k = chunk_by_index2(& mut flattened_with_index, max_bubble, 10);
    //
    //
    // tdsatda(&ddd, &flattened_with_index);


    // Take all intervals and check of relation
    // Take all intervals and get the stats
    // Stats -> Max, min size, number of traversals and intervals, number of children


    info!("BVD: Identify bubbles structures");

    // Make this stuff

    // else {
    //     // Index node->index - more in the function comment
    //     let (node_index_sort, node_to_index) = node2index_wrapper(&graph_f.paths, &threads);
    //     (intervals, bubbles) = bifurcation_bubble(&graph_f, &threads, node_index_sort, node_to_index);
    // }
    // info!("slicer dicer");
    // // intervals.sort();
    // // let mut test = Vec::new();
    // // for x in intervals.iter(){
    // //     if x.0 != 0{
    // //         break
    // //     }
    // //     test.push(x.clone());
    // // }
    // // info!("slicer dicer2");
    // //
    // // getSlice_test(&mut test, &graph_f.paths[0], &g2p[0]);
    // // // Lets write bubble and other file at the same time
    // // info!("Number of intervals {}", intervals.len().clone());
    // // info!("Number of bubbles {}", bubbles.len());
    // //let chunks2 = chunk_by_index2(intervals.clone(), bubbles.len().clone() as u32, threads as u32);
    //
    // //let chunks = chunk_by_index(&mut intervals, bubbles.len().clone() as u32, threads as u32);
    // info!("Statistics and writing output");
    // //
    // //let g2p = graph2pos(&graph_f);
    //
    // //
    // // Write output
    // let g2p = graph2pos(&graph_f);
    // solo_stats(&intervals[..],g2p, graph_f.paths.clone(), bubbles.clone());
    // //let f = graph_f.paths.clone();
    // //drop(graph_f);
    // //write_wrapper(chunks2, g2p, f, out_prefix, bubbles);
    // //
    // info!("Done");
    //
    //

}

