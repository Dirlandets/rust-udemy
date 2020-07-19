
mod part_2_types_and_variables;
mod part_3_control_flow;
mod part_4_data_structures;
mod part_5_standard_collections;

fn main() {
    println!("+++++ PART 2 TYPES AND VARIABLES +++++");
    part_2_types_and_variables::core_datatypes::run();
    part_2_types_and_variables::operators::run();
    part_2_types_and_variables::constants::run();
    part_2_types_and_variables::scope::run();
    part_2_types_and_variables::stuck_and_heap::run();
    println!("+++++ PART 3 CONTROL FLOW +++++");
    part_3_control_flow::if_statement::run();
    part_3_control_flow::loops::run();
    part_3_control_flow::match_statement::run();
    // part_3_control_flow::combination_lock::run(); 
    println!("+++++ PART 4 DATA STRUCTURES +++++");
    part_4_data_structures::structs::run();
    part_4_data_structures::enums::run();
    part_4_data_structures::unions::run();
    part_4_data_structures::option_if_let_while_let::run();
    part_4_data_structures::arrays::run();
    part_4_data_structures::tuples::run();
    part_4_data_structures::pattern_matching::run();
    part_4_data_structures::generics::run();
    println!("+++++ PART 5 Standard Collections +++++");
    part_5_standard_collections::overview::run();
    part_5_standard_collections::vector::run();

    // input::run();
}
