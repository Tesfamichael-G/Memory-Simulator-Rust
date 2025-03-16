use simulation_interface::mem_request::{MemoryCommand, Request, RequestType};

pub fn is_active_row_match(request: &Request, active_rows: &Vec<usize>) -> bool {
    active_rows.contains(&request.memory_address.row)
}

pub fn get_first_cmd(req: &Request) -> MemoryCommand {
    // Implement command determination logic
    // Example: return a command based on request type
    match req.request_type {
        RequestType::Read => MemoryCommand::Read,
        RequestType::Write => MemoryCommand::Write,
        _ => MemoryCommand::Read, // Default or error case
    }
}

pub fn hot_or_earliest(requests: &[Request], active_row: usize) -> Option<&Request> {
    fn hot_or_earlier<'a>(req1: &'a Request, req2: &'a Request, active_row: usize) -> &'a Request {
        let hit1 = req1.memory_address.row == active_row;
        let hit2 = req2.memory_address.row == active_row;

        match (hit1, hit2) {
            (true, false) => req1,
            (false, true) => req2,
            _ => if req1.stat.queued <= req2.stat.queued { req1 } else { req2 },
        }
    }

    if requests.is_empty() {
        return None;
    }

    let mut best_request = &requests[0];

    for request in &requests[1..] {
        best_request = hot_or_earlier(best_request, request, active_row);
    }

    Some(best_request)
}


//===================================================================
// fn select<'a>(&self, queue: &'a Q, dram: &'a D, bus_direction: BusDirection) -> Option<(&'a Request, MemoryCommand)> {
//     let mut best_request: Option<(&'a Request, MemoryCommand)> = None;
//
//     let cas_command = match bus_direction {
//         BusDirection::Read => MemoryCommand::Read,
//         BusDirection::Write => MemoryCommand::Write
//     };
//
//     for req in queue.get_requests(bus_direction) {
//         let command = dram.get_valid_command(req);
//         if command.is_none() {
//             continue;
//         }
//
//         let command = command.unwrap();
//
//         match best_request {
//             Some((best_req, best_cmd)) => {
//                 let should_replace = match (command, best_cmd) {
//                     (MemoryCommand::Read, MemoryCommand::Precharge) => true,
//                     (MemoryCommand::Write, MemoryCommand::Precharge) => true,
//                     (MemoryCommand::Read, MemoryCommand::Activate) => true,
//                     (MemoryCommand::Write, MemoryCommand::Activate) => true,
//
//                     (MemoryCommand::Activate, MemoryCommand::Precharge) => req.stat.queued < best_req.stat.queued,
//                     (MemoryCommand::Activate, MemoryCommand::Activate) => req.stat.queued < best_req.stat.queued,
//                     (MemoryCommand::Precharge, MemoryCommand::Precharge) => req.stat.queued < best_req.stat.queued,
//                     (MemoryCommand::Precharge, MemoryCommand::Activate) => req.stat.queued < best_req.stat.queued,
//
//                     (MemoryCommand::Read, MemoryCommand::Read) => req.stat.queued < best_req.stat.queued,
//                     (MemoryCommand::Write, MemoryCommand::Write) => req.stat.queued < best_req.stat.queued,
//
//                     _ => false,
//                 };
//
//                 if should_replace {
//                     best_request = Some((req, command));
//                 }
//             }
//             None => {
//                 // Initialize the best_request with the first valid request
//                 best_request = Some((req, command));
//             }
//         }
//     }
//
//     best_request.and_then(|req| dram.get_valid_command(req).map(|cmd| (req, cmd)))
// }

// fn select<'a>(&self, queue: &'a Q, dram: &'a D, bus_direction: BusDirection) -> Option<(&'a Request, MemoryCommand)> {
//     let mut best_request: Option<&'a Request = None;
//
//     for req in queue.get_requests(bus_direction) {
//         let request_status = dram.get_request_status(req);
//
//         let is_earliest = best_request
//             .map_or(true, |best| req.stat.queued < best.stat.queued);
//
//         if request_status.is_issuable && (request_status.is_hot || (is_earliest && !request_status.is_hot)) {
//             best_request = Some(req);
//         }
//     }
//
//     best_request.and_then(|req| dram.get_request_status(req).command.map(|cmd| (req, cmd)))
// }
// fn select<'a>(&self, queue: &'a Q, dram: &'a D, bus_direction: BusDirection) -> Option<(&'a Request, MemoryCommand)> {
//     let mut best_request: Option<&'a Request> = None;
//
//     for req in queue.get_requests(bus_direction) {
//         // let is_issuable = dram.can_issue(req);
//         // let is_hot = dram.is_hot(&req.memory_address);
//         let (is_issuable, is_hot) = dram.is_issuable_and_hot(req);
//
//         let is_earliest = best_request
//             .map_or(true, |best| req.stat.queued < best.stat.queued);
//
//         if is_issuable && (is_hot || (is_earliest && !is_hot)) {
//             best_request = Some(req);
//         }
//     }
//
//     if let Some(req) = best_request {
//         let state = dram.get_bank_state(&req.memory_address);
//         let active_row = dram.get_active_row(&req.memory_address);
//
//         let is_hot = matches!(active_row, Some(row) if row == req.memory_address.row);
//
//         let cmd = match (is_hot, state) {
//             (true, State::Active) => match req.request_type {
//                 RequestType::Read => MemoryCommand::Read,
//                 RequestType::Write => MemoryCommand::Write,
//             },
//             (false, State::Idle) => MemoryCommand::Activate,
//             (false, State::Active) => MemoryCommand::Precharge,
//             _ => return None,
//         };
//
//         Some((req, cmd))
//     } else {
//         None
//     }
// }

// fn select<'a>(&self, queue: &'a Q, dram: &'a D, bus_direction: BusDirection) -> Option<(&'a Request, MemoryCommand)> {
//     let mut best_request: Option<&'a Request> = None;
//
//     for req in queue.get_requests(bus_direction) {
//
//         let is_issuable = dram.can_issue(req);
//         let is_hot = dram.is_hot(&req.memory_address);
//
//         //match dram.get_active_row(&req.memory_address) {
//         //     Some(active_row) => active_row == req.memory_address.row,
//         //     None => false
//         // };
//
//         let is_earliest = match best_request {
//             Some(best) => req.stat.queued < best.stat.queued,
//             None => true,
//         };
//
//
//         if is_issuable && (is_hot || (is_earliest && !is_hot)) {
//             best_request = Some(req);
//         }
//     }
//
//     best_request.map(|req| {
//         let state = dram.get_bank_state(&req.memory_address);
//         let active_row = dram.get_active_row(&req.memory_address);
//
//         let is_hot = match active_row {
//             Some(row) => row == req.memory_address.row,
//             None => false,
//         };
//
//         let cmd = match (is_hot, state) {
//             (true, State::Active) => match req.request_type {
//                 RequestType::Read => MemoryCommand::Read,
//                 RequestType::Write => MemoryCommand::Write,
//             },
//             (false, State::Idle) => MemoryCommand::Activate,
//             (false, State::Active) => MemoryCommand::Precharge,
//             _ => return None,
//         };
//         Some((req, cmd))
//     }).flatten()
// }


// fn select<'a>(&self, queue: &'a Q, dram: &'a D, request_type: RequestType) -> Option<(&'a Request, MemoryCommand)> {
//
//     let requests = queue.get_requests(request_type);
//     if let Some(earliest) =  requests.iter().min_by_key(|req| req.stat.queued){
//
//     }
//
//     None
// }
// fn select<'a>(&self, queue: &'a Q, dram: &'a D, request_type: RequestType) -> Option<(&'a Request, MemoryCommand)> {
//     // Get all requests of the given type
//     let requests = queue.get_requests(request_type);
//
//     // Step 1: Find requests that match the active row
//     let mut matching_requests = Vec::new();
//     let mut non_matching_requests = Vec::new();
//
//     // Iterate over requests and categorize them
//     for req in requests {
//         let bank = dram.get_bank(&req.memory_address);
//         let active_row = bank.active_row(); // Assuming this method returns the active row for the bank
//
//         if req.memory_address.row == active_row {
//             matching_requests.push(req);
//         } else {
//             non_matching_requests.push(req);
//         }
//     }
//
//     // Step 2: If there are matching requests, select the one that was queued first
//     if !matching_requests.is_empty() {
//         let selected_request = matching_requests.iter()
//             .min_by_key(|req| req.stat.queued)
//             .expect("Matching request list should not be empty");
//
//         return Some((selected_request, MemoryCommand::Read)); // Assuming Read command, adjust as needed
//     }
//
//     // Step 3: If no matching requests, select the earliest among non-matching requests
//     if !non_matching_requests.is_empty() {
//         let selected_request = non_matching_requests.iter()
//             .min_by_key(|req| req.stat.queued)
//             .expect("Non-matching request list should not be empty");
//
//         return Some((selected_request, MemoryCommand::Read)); // Assuming Read command, adjust as needed
//     }
//
//     // If no requests available, return None
//     None
// }

