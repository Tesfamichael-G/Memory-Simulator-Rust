pub struct Count {
    // public long PRE { get; set; }
    pub pre: u32,
    // public long ACTS { get; set; }
    pub act:u32,
    // public long ACTS_RD { get; set; }
    pub act_for_read:u32,
    // public long ACTS_WR { get; set; }
    pub act_for_write:u32,
    // public long READS { get; set; }
    pub read:u32,
    // public long WRITES { get; set; }
    pub write:u32,
    // public long RD_HITS { get; set; }
    pub read_hits:u32,
    // public long WR_HITS { get; set; }
    pub write_hits:u32,
}