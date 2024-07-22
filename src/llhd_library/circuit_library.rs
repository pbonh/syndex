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
SPICENetlist = netlist_scope:NetlistScope;

@no_skip_ws
SubcircuitScope = i'.subckt' id:Identifier { ports:Identifier } EOL netlist_scope:NetlistScope \
     Ends;

NetlistScope = { elements:Element | statements:Statement | comments:Comment  | \
     subcircuit:SubcircuitScope};

@no_skip_ws
Comment = '*' {!'\n' char} EOL;

@no_skip_ws
Element = ( resistor:Resistor
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
          | mostransistor:MosTransistor ) EOL;

@no_skip_ws
Statement = ( model:ModelStatement
          | op:OpAnalysis
          | dc:DcAnalysis
          | tran:TransientAnalysis
          | ac:AcAnalysis
          | option:OptionStatement
          | option:PlotStatement
          | print:PrintStatement
          | End ) EOL;

Resistor = id:ResistorIdentifier Node Node Value { options:KeyValue };

@string
ResistorIdentifier = i'r' Node;

Capacitor = id:CapacitorIdentifier Node Node Value { options:KeyValue };

@string
CapacitorIdentifier = i'c' Node;

Inductor = id:InductorIdentifier Node Node Value { options:KeyValue };

@string
InductorIdentifier = i'l' Node;

MutualInductor1 = id:MutualInductorIdentifier Identifier Identifier Value;

MutualInductor2 = id:MutualInductorIdentifier Identifier Identifier i'k=' Value;

@string
MutualInductorIdentifier = i'k' Node;

VSwitch = id:VSwitchIdentifier Node Node Node Node Identifier;

@string
VSwitchIdentifier = i's' Node;

VoltageSource = id:VoltageSourceIdentifier p:Node n:Node { type:VoltageSourceType } { \
     values:SourceValues }+;

@string
VoltageSourceIdentifier = i'v' Node;

VoltageSourceType = 'dc' | 'ac';

CurrentSource = id:CurrentSourceIdentifier Node Node CurrentType;

@string
CurrentSourceIdentifier = i'i' Node;

VVoltageSource = id:VVoltageSourceIdentifier Node Node Node Node Value;

@string
VVoltageSourceIdentifier = i'e' Node;

VCurrentSource = id:VCurrentSourceIdentifier Node Node Node Node Value;

@string
VCurrentSourceIdentifier = i'g' Node;

CCurrentSource = id:CCurrentSourceIdentifier Node Node Identifier Value;

@string
CCurrentSourceIdentifier = i'f' Node;

Diode = id:DiodeIdentifier Node Node Identifier { DiodeParams };

@string
DiodeIdentifier = i'd' Node;

MosTransistor = id:MosTransistorIdentifier Node Node Node Node Identifier { options:KeyValue };

@string
MosTransistorIdentifier = i'm' Node;

ModelStatement = i'.model' Identifier Identifier ModelParams;

OpAnalysis = i'.op' { params:ParamValue | options:KeyValue }+;

DcAnalysis = i'.dc' 'src=' Identifier Value Value Value 'type=' ( 'lin' | 'log' );

TransientAnalysis = i'.tran' { timesteps:Value }+ { params:ParamValue | options:KeyValue };

AcAnalysis = i'.ac' ( 'lin' | 'log' ) Value Value Value;

OptionStatement = i'.options' id:Identifier { params:ParamValue | options:KeyValue }+;

PrintStatement = i'.print' id:Identifier { params:ParamValue | options:KeyValue }+;

PlotStatement = i'.plot' id:Identifier { params:ParamValue | options:KeyValue }+;

VoltageType = 'type=' ( 'vdc' 'vdc=' Value | 'vac' 'vac=' Value );

CurrentType = 'type=' ( 'idc' 'idc=' Value | 'iac' 'iac=' Value );

DiodeParams = { i'AREA=' Value } { i'T=' Value } { i'IC=' Value } { i'OFF=' Boolean };

MosParams = i'w=' Value i'l=' Value;

ModelParams = 'TYPE=' ( 'n' | 'p' ) { ModelParam };

ModelParam = 'TNOM=' Value | 'COX=' Value | 'GAMMA=' Value | 'NSUB=' Value | 'PHI=' Value | 'VTO=' \
     Value | 'KP=' Value | 'TOX=' Value | 'VFB=' Value | 'U0=' Value | 'TCV=' Value | 'BEX=' \
     Value;

SourceType = Identifier;

SourceValues = params:ParamValue | options:KeyValue | value:Value;

KeyValue = id:Identifier '=' value:Value;

ParamValue = id:Identifier '(' { value:Value }+ ')';

@string
@no_skip_ws
Node = {'a'..'z' | 'A'..'Z' | '_' | '0'..'9'};

@string
@no_skip_ws
Identifier = Letter {'a'..'z' | 'A'..'Z' | '_' | '0'..'9'};

@string
@no_skip_ws
Value = ( Digit | '+' | '-' ) { Digit | '.' | 'e' | 'E' | '-' | '+' | Letter };

@string
@no_skip_ws
Letter = 'a'..'z' | 'A'..'Z';

@string
@no_skip_ws
Digit = '0'..'9';

@string
@no_skip_ws
Unit = 'a' | 'f' | 'p' | 'n' | 'u' | 'm' | 'k' | 'm' | 'g' | 't' | 's';

@string
@no_skip_ws
Boolean = 'true' | 'false';

@string
@no_skip_ws
End = i'.end';

@string
@no_skip_ws
Ends = i'.ends';

@no_skip_ws
EOL = '\n' | ( '\r' '\n' );
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
        let netlist_scope = &ast.netlist_scope;
        println!("Comments: {:?}", netlist_scope.comments);
        println!("Statements: {:?}", netlist_scope.statements);
        // println!(
        //     "Options Statements: {:?}",
        //     netlist_scope.statements[0]
        //         .option
        //         .clone()
        //         .unwrap()
        //         .id
        //         .clone()
        // );
        println!("Elements: {:?}", netlist_scope.elements);
        assert_eq!(
            4,
            netlist_scope.comments.len(),
            "There should be 4 Comment in netlist."
        );
        assert_eq!(
            5,
            netlist_scope.statements.len(),
            "There should be 5 Statements in netlist."
        );
        assert_eq!(
            33,
            netlist_scope.elements.len(),
            "There should be 33 Elements in netlist."
        );
    }

    #[test]
    #[should_panic]
    fn spice_sky130_dk_a211o_2_example() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push(
            "resources/libraries_no_liberty/sky130_fd_sc_ls/latest/cells/a211o/\
             sky130_fd_sc_ls__a211o_2.spice",
        );
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let ast = SPICENetlist::parse(&spice_netlist_str).unwrap();
        let netlist_scope = &ast.netlist_scope;
        assert_eq!(
            15,
            netlist_scope.comments.len(),
            "There should be 15 Comment in netlist."
        );
        assert_eq!(
            0,
            netlist_scope.statements.len(),
            "There should be 0 Statements in netlist."
        );
        assert_eq!(
            0,
            netlist_scope.elements.len(),
            "There should be 0 Elements in netlist."
        );
        assert_eq!(
            1,
            netlist_scope.subcircuit.len(),
            "There should be 1 Subcircuits in netlist."
        );
        let _subcircuit1_scope = &netlist_scope.subcircuit[0].netlist_scope;
    }
}
