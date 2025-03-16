// pub
#[macro_export]
macro_rules! impl_ddr_transitions {
    ($model:ident, $rank_mod:ident, $bank_mod:ident, $bank_group_mod:ident) => {
        impl Transition for $model {
            fn get_transition_from_act(&self, component: Component) -> FromACT {
                match component {
                    Component::Rank => $rank_mod::get_all_transitions_from_act(&self),
                    Component::Bank => $bank_mod::get_all_transitions_from_act(&self),
                    Component::BankGroup => $bank_group_mod::get_all_transitions_from_act(&self),
                }
            }

            fn get_transition_from_pre(&self, component: Component) -> FromPRE {
                match component {
                    Component::Rank => $rank_mod::get_all_transitions_from_pre(&self),
                    Component::Bank => $bank_mod::get_all_transitions_from_pre(&self),
                    Component::BankGroup => $bank_group_mod::get_all_transitions_from_pre(&self),
                }
            }

            fn get_transition_from_prea(&self, component: Component) -> FromPREA {
                match component {
                    Component::Rank => $rank_mod::get_all_transitions_from_prea(&self),
                    Component::Bank => $bank_mod::get_all_transitions_from_prea(&self),
                    Component::BankGroup => $bank_group_mod::get_all_transitions_from_prea(&self),
                }
            }

            fn get_transition_from_rd(&self, component: Component) -> FromRD {
                match component {
                    Component::Rank => $rank_mod::get_all_transitions_from_rd(&self),
                    Component::Bank => $bank_mod::get_all_transitions_from_rd(&self),
                    Component::BankGroup => $bank_group_mod::get_all_transitions_from_rd(&self),
                }
            }

            fn get_transition_from_rda(&self, component: Component) -> FromRDA {
                match component {
                    Component::Rank => $rank_mod::get_all_transitions_from_rda(&self),
                    Component::Bank => $bank_mod::get_all_transitions_from_rda(&self),
                    Component::BankGroup => $bank_group_mod::get_all_transitions_from_rda(&self),
                }
            }

            fn get_transition_from_wr(&self, component: Component) -> FromWR {
                match component {
                    Component::Rank => $rank_mod::get_all_transitions_from_wr(&self),
                    Component::Bank => $bank_mod::get_all_transitions_from_wr(&self),
                    Component::BankGroup => $bank_group_mod::get_all_transitions_from_wr(&self),
                }
            }

            fn get_transition_from_wra(&self, component: Component) -> FromWRA {
                match component {
                    Component::Rank => $rank_mod::get_all_transitions_from_wra(&self),
                    Component::Bank => $bank_mod::get_all_transitions_from_wra(&self),
                    Component::BankGroup => $bank_group_mod::get_all_transitions_from_wra(&self),
                }
            }


        }
    };
}
