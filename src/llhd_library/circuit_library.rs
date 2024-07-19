mod peginator_doc_example;
mod peg_doc_example;

peg::parser!{
    grammar spice_parser() for str {
        rule identifier() -> &'input str
            = n:$(['a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9']*) { n }

        rule node() -> &'input str
            = identifier() / n:$(['0'..='9']+) { n }

        rule spice_value() -> &'input str
            = v:$(['0'..='9']+ ['a'..='z' | 'A'..='Z']?) { v }

        rule comment() -> ()
            = "*" [^'\n']* "\n" { () }

        rule continuation() -> ()
            = "+" [^'\n']* "\n" { () }

        rule resistor() -> ()
            = "R" identifier() node() node() spice_value() { () }

        rule capacitor() -> ()
            = "C" identifier() node() node() spice_value() ("ic=" spice_value())* { () }

        rule inductor() -> ()
            = "L" identifier() node() node() spice_value() ("ic=" spice_value())* { () }

        rule mutual_inductor1() -> ()
            = "K" identifier() identifier() identifier() spice_value() { () }

        rule mutual_inductor2() -> ()
            = "K" identifier() identifier() identifier() "k=" spice_value() { () }

        rule v_switch() -> ()
            = "S" identifier() node() node() node() node() identifier() { () }

        rule voltage_type() -> ()
            = "type=" ("vdc" "vdc=" spice_value() / "vac" "vac=" spice_value() ) { () }

        rule voltage_source() -> ()
            = "V" identifier() node() node() voltage_type() { () }

        rule current_type() -> ()
            = "type=" ("idc" "idc=" spice_value() / "iac" "iac=" spice_value() ) { () }

        rule current_source() -> ()
            = "I" identifier() node() node() current_type() { () }

        rule v_voltage_source() -> ()
            = "E" identifier() node() node() node() node() spice_value() { () }

        rule v_current_source() -> ()
            = "G" identifier() node() node() node() node() spice_value() { () }

        rule c_current_source() -> ()
            = "F" identifier() node() node() identifier() spice_value() { () }

        rule diode_params() -> ()
            = ("AREA=" spice_value()) ("T=" spice_value()) ("IC=" spice_value()) ("OFF=" ("true" / "false")) { () }

        rule diode() -> ()
            = "D" identifier() node() node() identifier() diode_params() { () }

        rule mos_params() -> ()
            = "w=" spice_value() "l=" spice_value() { () }

        rule mos_transistor() -> ()
            = "M" identifier() node() node() node() node() identifier() mos_params() { () }

        rule model_param() -> ()
            = "TNOM=" spice_value()
            / "COX=" spice_value()
            / "GAMMA=" spice_value()
            / "NSUB=" spice_value()
            / "PHI=" spice_value()
            / "VTO=" spice_value()
            / "KP=" spice_value()
            / "TOX=" spice_value()
            / "VFB=" spice_value()
            / "U0=" spice_value()
            / "TCV=" spice_value()
            / "BEX=" spice_value() { () }

        rule model_params() -> ()
            = "TYPE=" ("n" / "p") model_param()* { () }

        rule model_statement() -> ()
            = ".model" identifier() identifier() model_params() { () }

        rule op_analysis() -> ()
            = ".op" ("guess=" identifier()) { () }

        rule dc_analysis() -> ()
            = ".DC" "src=" identifier() spice_value() spice_value() spice_value() "type=" ("lin" / "log") { () }

        rule transient_analysis() -> ()
            = ".TRAN" "TSTEP=" spice_value() "TSTOP=" spice_value() ("TSTART=" spice_value() "UIC=" ("0" / "1" / "2" / "3") ("IC_LABEL=" identifier()) "METHOD=" identifier()) { () }

        rule ac_analysis() -> ()
            = ".AC" ("lin" / "log") spice_value() spice_value() spice_value() { () }

        rule key_spice_value() -> ()
            = identifier() "=" spice_value() { () }

        rule option_statement() -> ()
            = ".options" identifier() key_spice_value()? { () }

        rule print_statement() -> ()
            = ".print" key_spice_value()? { () }

        pub rule netlist() -> ()
            = (statement() / comment() / continuation())* { () }

        rule statement() -> ()
            = resistor()
            / capacitor()
            / inductor()
            / mutual_inductor1()
            / mutual_inductor2()
            / v_switch()
            / voltage_source()
            / current_source()
            / v_voltage_source()
            / v_current_source()
            / c_current_source()
            / diode()
            / mos_transistor()
            / model_statement()
            / op_analysis()
            / dc_analysis()
            / transient_analysis()
            / ac_analysis()
            / option_statement()
            / print_statement() { () }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn spice_netlist_example1() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push("resources/spice3f5_examples/mosamp2.cir");
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let ast = spice_parser::netlist(&spice_netlist_str).unwrap();
        println!("Parsed Result: {:?}", ast);
        // println!("Comments: {:?}", ast.comments);
        // println!("Statements: {:?}", ast.statements);
        // println!("Elements: {:?}", ast.elements);
        // assert_eq!(
        //     1,
        //     ast.comments.len(),
        //     "There should be 1 Comment in netlist."
        // );
        // assert_eq!(
        //     5,
        //     ast.statements.len(),
        //     "There should be 5 Statements in netlist."
        // );
        // assert_eq!(
        //     33,
        //     ast.elements.len(),
        //     "There should be 33 Elements in netlist."
        // );
    }
}
