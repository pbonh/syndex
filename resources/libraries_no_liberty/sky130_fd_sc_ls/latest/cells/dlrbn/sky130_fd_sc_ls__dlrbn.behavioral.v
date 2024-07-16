/*
 * Copyright 2020 The SkyWater PDK Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
*/


`ifndef SKY130_FD_SC_LS__DLRBN_BEHAVIORAL_V
`define SKY130_FD_SC_LS__DLRBN_BEHAVIORAL_V

/**
 * dlrbn: Delay latch, inverted reset, inverted enable,
 *        complementary outputs.
 *
 * Verilog simulation functional model.
 */

`timescale 1ns / 1ps
`default_nettype none

// Import user defined primitives.
`include "../../models/udp_dlatch_pr_pp_pg_n/sky130_fd_sc_ls__udp_dlatch_pr_pp_pg_n.v"

`celldefine
module sky130_fd_sc_ls__dlrbn (
    Q      ,
    Q_N    ,
    RESET_B,
    D      ,
    GATE_N
);

    // Module ports
    output Q      ;
    output Q_N    ;
    input  RESET_B;
    input  D      ;
    input  GATE_N ;

    // Module supplies
    supply1 VPWR;
    supply0 VGND;
    supply1 VPB ;
    supply0 VNB ;

    // Local signals
    wire RESET          ;
    wire intgate        ;
    reg  notifier       ;
    wire D_delayed      ;
    wire GATE_N_delayed ;
    wire RESET_delayed  ;
    wire RESET_B_delayed;
    wire buf_Q          ;
    wire awake          ;
    wire cond0          ;
    wire cond1          ;

    //                                     Name     Output   Other arguments
    not                                    not0    (RESET  , RESET_B_delayed                                );
    not                                    not1    (intgate, GATE_N_delayed                                 );
    sky130_fd_sc_ls__udp_dlatch$PR_pp$PG$N dlatch0 (buf_Q  , D_delayed, intgate, RESET, notifier, VPWR, VGND);
    assign awake = ( VPWR === 1'b1 );
    assign cond0 = ( awake && ( RESET_B_delayed === 1'b1 ) );
    assign cond1 = ( awake && ( RESET_B === 1'b1 ) );
    buf                                    buf0    (Q      , buf_Q                                          );
    not                                    not2    (Q_N    , buf_Q                                          );

endmodule
`endcelldefine

`default_nettype wire
`endif  // SKY130_FD_SC_LS__DLRBN_BEHAVIORAL_V