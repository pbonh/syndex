mod peginator_doc_example;

use peginator_macro::peginate;

// SPICE EBNF(Extended Backus Naur Form)
// netlist         = { element };
//
// element         = resistor
//                | capacitor
//                | inductor
//                | mutual_inductor
//                | voltage_controlled_switch
//                | voltage_source
//                | current_source
//                | voltage_controlled_voltage_source
//                | voltage_controlled_current_source
//                | current_controlled_current_source
//                | diode
//                | mos_transistor;
// resistor        = "R", identifier, node, node, value;
// capacitor       = "C", identifier, node, node, value, [ "ic=", value ];
// inductor        = "L", identifier, node, node, value, [ "ic=", float ];
// mutual_inductor = "K", identifier, identifier, identifier, ( value | "k=", value );
// voltage_controlled_switch
//                = "S", identifier, node, node, node, node, model_id;
// voltage_source  = "v", identifier, node, node, { type_value };
// current_source  = "i", identifier, node, node, { type_value };
// voltage_controlled_voltage_source
//                = "E", identifier, node, node, node, node, value;
// voltage_controlled_current_source
//                = "G", identifier, node, node, node, node, value;
// current_controlled_current_source
//                = "F", identifier, node, node, identifier, value;
// diode           = "D", identifier, node, node, model_id, { diode_param };
// mos_transistor  = "M", identifier, node, node, node, node, model_id, "w=", float, "l=", float;
// identifier      = letter, { letter | digit };
// node            = letter, { letter | digit };
// value           = float;
// type_value      = "type=", type_identifier, type_identifier, "=", float;
// type_identifier = "vdc" | "vac" | "idc" | "iac" | ... ;
// diode_param     = ( "AREA=", float | "T=", float | "IC=", float | "OFF=", boolean );
// model_id        = identifier;
// letter          = "a" | "b" | "c" | ... | "z" | "A" | "B" | "C" | ... | "Z";
// digit           = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";
// float           = digit, { digit }, [ ".", { digit } ];
// boolean         = "true" | "false";

// peginate!(
//     "
// @export
// SPICENetlist = { statements:Statement };
// Statement    = comments:Comments | elements:Elements | commands:Command;
// Elements     = Resistor
//                 | Capacitor
//                 | Inductor
//                 | MutualInductor
//                 | VoltageControlledSwitch
//                 | VoltageSource
//                 | CurrentSource
//                 | VoltageControlledVoltageSource
//                 | VoltageControlledCurrentSource
//                 | CurrentControlledCurrentSource
//                 | Diode
//                 | MOSTransistor;
// 
// @no_skip_ws
// Comments         = '*' {!'\n' char} '\n';
// 
// Command          = '.' ( option:Option
//                         | transient:Transient
//                         | print:Print
//                         | plot:Plot
//                         | end:End );
// 
// Option           = i'options' { OptionArguments };
// Transient        = i'tran' { OptionArguments };
// Print            = i'print' { OptionArguments };
// Plot             = i'plot' { OptionArguments };
// OptionArguments  = modelId:Identifier | value:OptionExpression | assignment:OptionAssignment;
// OptionAssignment = Identifier '=' { value:OptionValue };
// OptionExpression = Identifier '(' { value:OptionValue } ')';
// OptionValue      =  UnitValue | Identifier ;
// @string
// End              = i'end';
// 
// Resistor        = i'R' Identifier Node Node Value;
// Capacitor       = i'C' Identifier Node Node Value [ i'ic=' Value ];
// Inductor        = i'L' Identifier Node Node Value [ i'ic=' Value ];
// MutualInductor  = i'K' Identifier Identifier Identifier ( Value | i'k=' Value );
// VoltageControlledSwitch =
//                     i'S' Identifier Node Node Node Node ModelId;
// VoltageSource   = i'v' Identifier Node Node { TypeValue };
// CurrentSource   = i'i' Identifier Node Node { TypeValue };
// VoltageControlledVoltageSource =
//                     i'E' Identifier Node Node Node Node Value;
// VoltageControlledCurrentSource =
//                     i'G' Identifier Node Node Node Node Value;
// CurrentControlledCurrentSource =
//                     i'F' Identifier Node Node Identifier Value;
// Diode           = i'D' Identifier Node Node ModelId { DiodeParam };
// MOSTransistor   = i'M' Identifier Node Node Node Node ModelId i'w=' Num i'l=' Num;
// 
// Node            = Identifier;
// Identifier      = Letter { Letter | Digit };
// Value           = UnitValue;
// TypeValue       = i'type=' TypeIdentifier TypeIdentifier '=' Num;
// TypeIdentifier  = 'vdc' | 'vac' | 'idc' | 'iac';
// DiodeParam      = ( 'AREA=' Num | 'T=' Num | 'IC=' Num | 'OFF=' Boolean );
// ModelId         = Identifier;
// Num           = Digit [ Digit ] [ '.' { Digit } ];
// UnitValue       = Digit [ Digit ] [ '.' { Digit } ] [ Letter ];
// @string
// Letter          = 'a'..'z' | 'A'..'Z';
// @string
// Digit           = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
// @string
// Boolean         = i'true' | i'false';
// "
// );

peginate!(
    "
@export
SPICENetlist = { elements:Element | statements:Statement | comments:Comment };

@no_skip_ws
Comment = '*' {!'\n' char} '\n';

Element = resistor:Resistor
          | capacitor:Capacitor
          | inductor:Inductor
          | mutualinductor1:MutualInductor1
          | mutualinductor2:MutualInductor2
          | vswitch:VSwitch
          | voltagesource:VoltageSource
          | currentsource:CurrentSource
          | vvoltagesource:VVoltageSource
          | vcurrentsource:VCurrentSource
          | ccurrentsource:CCurrentSource
          | diode:Diode
          | mostransistor:MosTransistor;

Statement = model:ModelStatement
          | op:OpAnalysis
          | dc:DcAnalysis
          | tran:TransientAnalysis
          | ac:AcAnalysis
          | option:OptionStatement
          | print:PrintStatement;

Resistor = i'R' Identifier Node Node Value;

Capacitor = i'C' Identifier Node Node Value [ 'ic=' Value ];

Inductor = i'L' Identifier Node Node Value [ 'ic=' Value ];

MutualInductor1 = i'K' Identifier Identifier Identifier Value;

MutualInductor2 = i'K' Identifier Identifier Identifier 'k=' Value;

VSwitch = i'S' Identifier Node Node Node Node Identifier;

VoltageSource = i'V' Identifier Node Node VoltageType;

CurrentSource = i'I' Identifier Node Node CurrentType;

VVoltageSource = i'E' Identifier Node Node Node Node Value;

VCurrentSource = i'G' Identifier Node Node Node Node Value;

CCurrentSource = i'F' Identifier Node Node Identifier Value;

Diode = i'D' Identifier Node Node Identifier [ DiodeParams ];

MosTransistor = i'M' Identifier Node Node Node Node Identifier MosParams;

ModelStatement = i'.model' Identifier Identifier ModelParams;

OpAnalysis = i'.op' { KeyValue };

DcAnalysis = i'.DC' 'src=' Identifier Value Value Value 'type=' ( 'lin' | 'log' );

TransientAnalysis = i'.TRAN' 'TSTEP=' Value 'TSTOP=' Value [ 'TSTART=' Value 'UIC=' ( '0' | '1' | '2' | '3' ) [ 'IC_LABEL=' Identifier ] 'METHOD=' Identifier ];

AcAnalysis = i'.AC' ( 'lin' | 'log' ) Value Value Value;

OptionStatement = i'.options' Identifier { KeyValue };

PrintStatement = i'.print' { KeyValue };

Identifier = Letter { Letter | Digit };

Node = Identifier | Digit { Digit };

Value = Digit { Digit | '.' | 'e' | 'E' | '-' | '+' | Unit };

VoltageType = 'type=' ( 'vdc' 'vdc=' Value | 'vac' 'vac=' Value );

CurrentType = 'type=' ( 'idc' 'idc=' Value | 'iac' 'iac=' Value );

DiodeParams = [ 'AREA=' Value ] [ 'T=' Value ] [ 'IC=' Value ] [ 'OFF=' Boolean ];

MosParams = i'w=' Value i'l=' Value;

ModelParams = 'TYPE=' ( 'n' | 'p' ) [ ModelParam ];

ModelParam = 'TNOM=' Value | 'COX=' Value | 'GAMMA=' Value | 'NSUB=' Value | 'PHI=' Value | 'VTO=' Value | 'KP=' Value | 'TOX=' Value | 'VFB=' Value | 'U0=' Value | 'TCV=' Value | 'BEX=' Value;

KeyValue = Identifier '=' Value;

@string
Letter = 'a'..'z' | 'A'..'Z';

@string
Digit = '0'..'9';

@string
Unit = 'a' | 'f' | 'p' | 'n' | 'u' | 'm' | 'k' | 'meg' | 'g' | 't';

@string
Boolean = 'true' | 'false';

@string
EOL = '\n' | '\r' '\n';
"
);


#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use peginator::PegParser;

    use super::*;

    #[test]
    fn spice_netlist_example1() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push("resources/spice3f5_examples/mosamp2.cir");
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let ast = SPICENetlist::parse(&spice_netlist_str).unwrap();
        println!("Comments: {:?}", ast.comments);
        println!("Statements: {:?}", ast.statements);
        println!("Elements: {:?}", ast.elements);
        assert_eq!(
            1,
            ast.comments.len(),
            "There should be 1 Comment in netlist."
        );
        assert_eq!(
            5,
            ast.statements.len(),
            "There should be 5 Statements in netlist."
        );
        assert_eq!(
            33,
            ast.elements.len(),
            "There should be 33 Elements in netlist."
        );
    }
}
