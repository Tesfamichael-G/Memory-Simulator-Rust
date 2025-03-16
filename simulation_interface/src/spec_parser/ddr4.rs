use serde_json::Value;
use crate::address_mapping::MappingBits;
use crate::memory_model::ddr::{DDR4, Org};
use crate::spec_parser::SpecParser;

impl SpecParser for DDR4{
    type Model = DDR4;

    fn parse(spec_path: &str) -> Result<(Org, Self::Model, MappingBits), serde_json::Error> {

        let json_string = std::fs::read_to_string(spec_path)
            .expect("error reading json string for mem spec");

        let json_value: Value = serde_json::from_str(&json_string)?;
        let timing_obj = json_value.get("timing").expect("JSON Error getting timing parameters");

        let timing_fields = DDR4 {
            rate: timing_obj["rate"].as_i64().unwrap() as u16,
            freq: timing_obj["freq"].as_i64().unwrap() as u16, // Assuming freq should be u16
            t_ck: timing_obj["tck"].as_f64().unwrap() as f32,
            bl: timing_obj["bl"].as_i64().unwrap() as u16,
            cl: timing_obj["cl"].as_i64().unwrap() as u16,
            rcd: timing_obj["rcd"].as_i64().unwrap() as u16,
            rp: timing_obj["rp"].as_i64().unwrap() as u16,
            ras: timing_obj["ras"].as_i64().unwrap() as u16,
            rc: timing_obj["rc"].as_i64().unwrap() as u16,
            wr: timing_obj["wr"].as_i64().unwrap() as u16,
            rtp: timing_obj["rtp"].as_i64().unwrap() as u16,
            cwl: timing_obj["cwl"].as_i64().unwrap() as u16,
            rtrs: timing_obj["rtrs"].as_i64().unwrap() as u16,
            xp: timing_obj["xp"].as_i64().unwrap() as u16,
            ckesr: timing_obj["ckesr"].as_i64().unwrap() as u16,
            cke: timing_obj["cke"].as_i64().unwrap() as u16,

            ccdl: timing_obj["ccdl"].as_i64().unwrap() as u16,
            ccds: timing_obj["ccds"].as_i64().unwrap() as u16,

            faw: timing_obj["faw"].as_i64().unwrap() as u16,

            refi: timing_obj["refi"].as_i64().unwrap() as u32,
            rfc: timing_obj["rfc"].as_i64().unwrap() as u16,

            rfc2: timing_obj["rfc2"].as_i64().unwrap() as u16,
            rfc4: timing_obj["rfc4"].as_i64().unwrap() as u16,

            rrdl: timing_obj["rrdl"].as_i64().unwrap() as u16,
            rrds: timing_obj["rrds"].as_i64().unwrap() as u16,

            wtrl: timing_obj["wtrl"].as_i64().unwrap() as u16,
            wtrs: timing_obj["wtrs"].as_i64().unwrap() as u16,

            xs: timing_obj["xs"].as_i64().unwrap() as u16,

        };

        let org_obj = json_value.get("organization").expect("JSON Error getting organization parameters");

        let org_fields = Org {
            size: org_obj["size"].as_i64().unwrap() as u32,
            dq: org_obj["dq"].as_i64().unwrap() as u8,
            channel: org_obj["channel"].as_i64().unwrap() as u8,
            rank: org_obj["rank"].as_i64().unwrap() as u8,
            bank_group: org_obj["bank_group"].as_i64().unwrap() as u8,
            bank: org_obj["bank"].as_i64().unwrap() as u8,
            row: org_obj["row"].as_i64().unwrap() as u32,
            column: org_obj["column"].as_i64().unwrap() as u16,
        };
        // println!("org fields: {:?}", org_fields);
        let mut mapping_bits = org_fields.default_mapping_bits();

        let ddr4_prefetch_size:u8 = 8;
        let channel_width:u8 = 64;
        let tx_bytes:u32 = ddr4_prefetch_size as u32 * channel_width as u32 / 8u32;

        mapping_bits.col_bits -= f64::log2(ddr4_prefetch_size as f64).ceil() as usize;
        mapping_bits.tx_offset = f64::log2(tx_bytes as f64 + 1.0).ceil() as usize;


        println!("Mapping bits: {:?}", mapping_bits);
        Ok((org_fields, timing_fields, mapping_bits))
    }


}
