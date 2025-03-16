// use crate::memory_model::ddr::transitions::from_cas::{FromRD, FromRDA, FromWR, FromWRA};
// use crate::memory_model::ddr::transitions::from_ras::{FromACT, FromPRE, FromPREA};
//
// pub enum Component {
//     Rank,
//     Bank,
//     BankGroup,
// }
// pub trait Transition {
//     fn get_transition_from_act(&self, component: Component) -> FromACT;
//     fn get_transition_from_pre(&self, component: Component) -> FromPRE;
//     fn get_transition_from_prea(&self, component: Component) -> FromPREA;
//     fn get_transition_from_rd(&self, component: Component) -> FromRD;
//     fn get_transition_from_rda(&self, component: Component) -> FromRDA;
//     fn get_transition_from_wr(&self, component: Component) -> FromWR;
//     fn get_transition_from_wra(&self, component: Component) -> FromWRA;
// }
//
// pub struct TransitionTiming{
//     pub from_act: FromACT,
//     pub from_pre: FromPRE,
//     pub from_prea: FromPREA,
//     pub from_rd: FromRD,
//     pub from_rda: FromRDA,
//     pub from_wr: FromWR,
//     pub from_wra: FromWRA,
// }
//
// pub struct RankTransitionTiming(TransitionTiming);
// pub struct BankGroupTransitionTiming(TransitionTiming);
// pub struct BankTransitionTiming(TransitionTiming);
//
// impl TransitionTiming{
//
//     pub fn from_model<T>(model: T, component: Component) -> TransitionTiming {
//
//         TransitionTiming {
//             from_act: model.get_transition_from_act(component),
//             from_pre: model.get_transition_from_pre(component),
//             from_prea: model.get_transition_from_prea(component),
//             from_rd: model.get_transition_from_rd(component),
//             from_rda: model.get_transition_from_rda(component),
//             from_wr: model.get_transition_from_wr(component),
//             from_wra: model.get_transition_from_wra(component),
//         }
//
//     }
//
// }
