/**
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

`ifndef SKY130_FD_SC_LS__CONB_TB_V
`define SKY130_FD_SC_LS__CONB_TB_V

/**
 * conb: Constant value, low, high outputs.
 *
 * Autogenerated test bench.
 *
 * WARNING: This file is autogenerated, do not modify directly!
 */

`timescale 1ns / 1ps
`default_nettype none

`include "sky130_fd_sc_ls__conb.v"

module top();

    // Inputs are registered
    reg VPWR;
    reg VGND;
    reg VPB;
    reg VNB;

    // Outputs are wires
    wire HI;
    wire LO;

    initial
    begin
        // Initial state is x for all inputs.
        VGND = 1'bX;
        VNB  = 1'bX;
        VPB  = 1'bX;
        VPWR = 1'bX;

        #20   VGND = 1'b0;
        #40   VNB  = 1'b0;
        #60   VPB  = 1'b0;
        #80   VPWR = 1'b0;
        #100  VGND = 1'b1;
        #120  VNB  = 1'b1;
        #140  VPB  = 1'b1;
        #160  VPWR = 1'b1;
        #180  VGND = 1'b0;
        #200  VNB  = 1'b0;
        #220  VPB  = 1'b0;
        #240  VPWR = 1'b0;
        #260  VPWR = 1'b1;
        #280  VPB  = 1'b1;
        #300  VNB  = 1'b1;
        #320  VGND = 1'b1;
        #340  VPWR = 1'bx;
        #360  VPB  = 1'bx;
        #380  VNB  = 1'bx;
        #400  VGND = 1'bx;
    end

    sky130_fd_sc_ls__conb dut (.VPWR(VPWR), .VGND(VGND), .VPB(VPB), .VNB(VNB), .HI(HI), .LO(LO));

endmodule

`default_nettype wire
`endif  // SKY130_FD_SC_LS__CONB_TB_V
