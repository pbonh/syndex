pub mod circuit_library;
/// Parser for SPICE Netlist
///
/// Syntax
///
/// General Elements
/// In general, an element is declared with the following general syntax:
///
/// General syntax:
/// <K><description_string> <n1> <n2> [value] [<option>=<value>] [...] ...
///
/// Where:
///
/// <K> is a character, a unique identifier for each type of element (e.g. R for resistor).
/// <description_string> is a string without spaces (e.g. 1).
/// <n1>, a string, is the node of the circuit to which the anode of the element is connected.
/// <n2>, a string, is the node of the circuit to which the cathode of the element is connected.
/// [value] if supported, is the ‘value’ of the element, in mks (e.g. R1 1 0 500k)
/// <option>=<value> are the parameters of the element.
/// Nodes may have any label, without spaces, except the reference node (GND) which has to be 0.
///
/// Linear elements
///
/// Resistors
///
/// General syntax:
/// R<string> n1 n2 <value>
///
/// n1 and n2 are the element nodes.
/// value is the element resistance. It may any non-zero value (negative values are supported too).
/// Example:
/// R1 1 0 1k
/// RAb_ input output 1.2e6
///
/// Capacitors
///
/// General syntax:
/// C<string> n1 n2 <value> [ic=<value>]
///
/// n1 and n2 are the element nodes.
/// value is the capacitance in Farads.
/// ic=<value> is an optional attribute that can be set to provide an initial value voltage value for a transient simulation. See also the discussion of the UIC parameter in TRAN simulations.
/// Example:
/// C1 1 0 1u
/// Cfeedback out+ in- 1e6
///
/// Inductors
///
/// General syntax:
/// L<string> n1 n2 <value> [ic=<float>]
///
/// n1 and n2 are the element nodes.
/// value is the inductance in Henry.
/// ic=<value> is an optional attribute that can be set to provide an initial value for a transient simulation. See also the discussion of the UIC parameter in TRAN simulations.
/// Example:
/// L1 1 0 1u
/// Lchoke inA inB 1e6
///
/// Mutual Inductors
///
/// General syntax:
/// Either:
/// K<string> <inductor1> <inductor2> <value>
/// or
/// K<string> <inductor1> <inductor2> k=<value>
///
/// <inductor1> and <inductor2> are the coupled inductors. They need to be specified before the coupling can be inserted.
/// value is the coupling factor, k. It is a needs to be less than 1.
/// Example:
/// L1 1 0 1u
/// L2 3 4 5u
/// K1 L1 L2 0.6
///
/// Voltage-controlled switch
///
/// General syntax:
/// S<string> n1 n2 ns1 ns2 <model_id>
///
/// n1 and n2 are the nodes corresponding to the output port, where the switch opens and closes the connection.
/// ns1 and ns2 are the nodes corresponding to the driving port, where the voltage setting the switch status is read.
/// model_id is the model describing the switch operation. Notice that even if an ideal switch is a (piece-wise) linear element, its model implementation may not be, depending on the implementation details of the transition region.
///
/// Independent sources
///
/// Voltage source
///
/// General syntax:
/// v<string> n1 n2 [type=vdc vdc=float] [type=vac vac=float] [type=....]
///
/// Where the third type (if added) is one of: sin, pulse, exp, sffm, am.
///
/// Current source
///
/// General syntax:
/// i<string> n1 n2 [type=idc idc=float] [type=iac iac=float] [type=....]
///
/// The declaration of the time variant part is the same as for voltage sources, except that vo becomes io, va becomes ia and so on.
///
/// Dependent sources
///
/// Voltage-Controlled Voltage Source (VCVS)
///
/// General syntax:
/// E<string> n+ n- ns+ ns- <value>
///
/// n+ and n- are the nodes corresponding to the output port, where the voltage is forced.
/// ns+ and ns- are the nodes corresponding to the driving port, where the voltage is read.
/// value is the proportionality factor, i.e.: V(n+) - V(n-) = value*[V(sn+) - V(sn-)].
///
/// Voltage-Controlled Current Source (VCCS)
///
/// General syntax:
/// G<string> n+ n- ns+ ns- <value>
///
/// n+ and n- are the nodes corresponding to the output port, where the current is forced.
/// ns+ and ns- are the nodes corresponding to the driving port, where the voltage is read.
/// value is the proportionality factor, i.e.: I(n+,n-) = value*[V(sn+) - V(sn-)].
///
/// Current-Controlled Current Source (CCCS)
///
/// General syntax:
/// F<string> n+ n- <voltage_source> <value>
///
/// n+ and n- are the nodes corresponding to the output port, where the current is forced.
/// voltage_source is the ID of a voltage source whose current controls the dependent current source. It must exist in the circuit. Note that netlists are case-insensitive, i.e. Va is the same as vA.
/// value is the proportionality factor, i.e.: I(n+,n−)=value∗I[<voltagesource>]
/// .
/// Non-linear elements
///
/// The simulator has a few non-linear components built-in. Others may easily be added as external modules.
///
/// Diode
///
/// General syntax:
/// D<string> n1 n2 <model_id> [<AREA=float> <T=float> <IC=float> <OFF=boolean>]
///
/// Parameters:
/// n1: anode.
/// n2: cathode.
/// <model_id>: the ID of the diode model.
/// AREA: The area of the PN junction.
/// T: the temperature of operation, if different from the circuit temperature.
/// IC: initial condition statement (voltage).
/// OFF: Consider the diode to be initially off in transient analyses.
///
/// MOS Transistors
///
/// General syntax:
/// M<string> nd ng ns nb <model_id> w=<float> l=<float>
///
/// A MOS device declaration requires:
/// nd: the drain node,
/// ng: the gate node,
/// ns: the source node,
/// nb: the bulk node.
/// <model_id>: is a string that links this device to a .model declaration in the netlist. The model is actually responsible of the operation of the device.
/// w: gate width, in meters.
/// l: gate length, in meters.
///
/// User-defined elements
///
/// General syntax:
/// Y<X> <n1> <n2> module=<module_name> type=<type> [<param1>=<value1> ...]
///
/// Ahkab can parse user-defined elements. In order for this to work, you should write a Python module that supplies the element class. The simulator will attempt to load the module <module_name> and it will then look for a class named <type> within.
/// See netlist_parser.parse_elem_user_defined() for further information.
///
/// Subcircuit calls
///
/// General syntax:
/// X<string> name=<subckt_label> [<subckt_node1>=<node_a> <subckt_node2>=<node_b> ... ]
///
/// Insert a subcircuit, connected as specified.
/// All nodes in the subcircuit specification must be connected to a circuit node. The call can be placed before or after the corresponding .subckt directive.
///
/// Time functions
/// Time functions may be used in conjunction with an independent source to define its time-dependent behavior.
/// This is typically done adding a type=... section in the element declaration, such as:
/// V1 1 2 vdc=10m type=sin VO=10m VA=1.2 FREQ=500k TD=1n THETA=0
///
/// Sinusoidal waveform
/// A damped sinusoidal time function.
/// It may be described with the syntax:
/// type=sin <VO> <VA> <FREQ> <TD> <THETA> <PHASE>
/// or with the more verbose variant:
/// type=sin VO=<float> VA=<float> FREQ=<float> TD=<float> THETA=<float> PHASE=<float>
/// Mathematically described by:
/// When t<td
/// :
/// V(t)=VO
/// When t≥td
/// :
/// V(t)=VO+VA⋅exp[−THETA⋅(t−TD)]⋅sin[2πFREQ(t−TD)+(PHASE/360)]
/// Where:
/// VO
///  is the offset voltage in Volt.
/// VA
///  is the amplitude in Volt.
/// FREQ
///  is the frequency in Hertz.
/// TD
///  is the delay in seconds.
/// THETA
///  is the damping factor per second.
/// PHASE
///  is the phase in degrees.
///
/// Exponential source
/// An exponential waveform may be described with one of the following syntaxes:
/// type=EXP <V1> <V2> <TD1> <TAU1> [<TD2> <TAU2>]
/// type=exp v1=<float> v2=float td1=float tau1=<float> td2=<float> tau2=<float>
/// Example:
/// VIN input 0 type=vdc vdc=0 type=exp 4 1 2n 30n 60n 40n
/// Mathematically, it is described by the equations:
/// 0≤t<TD1
/// :
/// f(t)=V1
/// TD1<t<TD2
/// f(t)=V1+(V2−V1)⋅[1−exp(−t−TD1TAU1)]
/// t>TD2
/// f(t)=V1+(V2−V1)⋅[1−exp(−t−TD1TAU1)]+(V1−V2)⋅[1−exp(−t−TD2TAU2)]
/// Parameters:
/// Parameter	Meaning	Default value	Units
/// V1	initial value	 	V or A
/// V2	pulsed value	 	V or A
/// TD1	rise delay time	0.0	s
/// TAU1	rise time constant	 	s
/// TD2	fall delay time	Infinity	s
/// TAU2	fall time constant	Infinity	s
/// Pulsed source
/// A square wave.
/// type=pulse v1=<float> v2=<float> td=<float> tr=<float> tf=<float> pw=<float> per=<float>
/// or:
/// PULSE <V1> <V2> <TD> <TR> <TF> <PW> <PER>
/// Parameters:
/// Parameter	Meaning	Default value	Units
/// V1	first value	 	V or A
/// V2	second value	 	V or A
/// TD	delay time	0.0	s
/// TR	rise time	 	s
/// TF	fall time	 	s
/// PW	pulse width	 	s
/// PER	periodicity interval	 	s
/// Single-Frequency Frequency Modulation (SFFM)
/// A SFFM wave.
/// It may be described with any of the following syntaxes:
/// TYPE=sffm <VO> <VA> <FC> <MDI> <FS> [<TD>]
/// or
/// type=sffm vo=<float> v=<float> f=<float> md=<float> f=<float> +
/// [td=<float>]
/// Mathematically, it is described by the equations:
/// 0≤t≤tD
/// :
/// f(t)=VO
/// t>tD
/// f(t)=VO+VA⋅sin[2πfC(t−tD)+MDIsin[2πfS(t−tD)]]
/// Parameters:
/// Parameter	Meaning	Default value	Units
/// VO	offset	 	V or A
/// VA	amplitude	 	V or A
/// FC	carrier frequency	 	Hz
/// MDI	modulation index
/// FS	signal frequency	 	HZ
/// TD	time delay	0.0	s
/// Amplitude Modulation (AM)
/// An AM waveform.
/// It may be described with any of the following syntaxes:
/// TYPE=AM <SA> <OC> <FM> <FC> [<TD>]
/// or
/// type=am sa=<float> oc=<float> fm=<float> fc=<float> [td=<float>]
/// Mathematically, it is described by the equations:
/// 0≤t≤tD
/// :
/// f(t)=O
/// t>tD
/// f(t)=SA⋅[OC+sin[2πfm(t−tD)]]⋅sin[2πfc(t−tD)]
/// Parameters:
/// Parameter	Meaning	Default value	Units
/// SA	amplitude	 	V or A
/// FC	carrier frequency	 	Hz
/// FM	modulation frequency	 	Hz
/// OC	offset constant
/// TD	time delay	0.0	s
///
/// Device models
///
/// Rudimentary EKV 3.0 MOS model
///
/// General syntax:
/// .model ekv <model_id> TYPE=<n/p> [TNOM=<float> COX=<float> GAMMA=<float> NSUB=<float> PHI=<float> VTO=<float> KP=<float> TOX=<float> VFB=<float> U0=<float> TCV=<float> BEX=<float>]
///
/// The EKV model was developed by Matthias Bucher, Christophe Lallement, Christian Enz, Fabien Théodoloz, François Krummenacher at the Electronics Laboratories, Swiss Federal Institute of Technology (EPFL), Lausanne, Switzerland.
/// It is described here:
/// rev. 2.6 - http://legwww.epfl.ch/ekv/pdf/ekv_v262.pdf
/// rev. 3.0 - http://www.nsti.org/publications/MSM/2002/pdf/346.pdf
/// The authors are in no way responsible for any bug that may be present in my implementation. :)
/// The model is missing:
/// channel length modulation,
/// complex mobility reduction,
/// RSCE transcapacitances,
/// the quasistatic modeling.
/// It does identify weak, moderate and strong inversion zones, it is fully symmetrical, it treats N and P devices equally.
///
/// Square-law MOS model
///
/// General syntax:
/// .model mosq <model_id> TYPE=<n/p> [TNOM=<float> COX=<float> GAMMA=<float> NSUB=<float> PHI=<float> VTO=<float> KP=<float> TOX=<float> VFB=<float> U0=<float> TCV=<float> BEX=<float>]
///
/// This is a square-law MOS model without velocity saturation (and second order effects like punch-through and such).
///
/// DIODE model
///
/// General syntax:
/// .model diode <model_id> [IS=<float> N=<float> ISR=<float> NR=<float> RS=<float> CJ0=<float> M=<float> VJ=<float> FC=<float> CP=<float> TT=<float> BV=<float> IBV=<float> KF=<float> AF=<float> FFE=<float> TEMP=<float> XTI=<float> EG=<float> TBV=<float> TRS=<float> TTT1=<float> TTT2=<float> TM1=<float> TM2=<float>]
///
/// The diode model implements the Shockley diode equation. Currently the capacitance modeling part is missing.
/// The most important parameters are:
/// Parameter	Default value	Description
/// IS	1e-14 A	Specific current
/// N	1.0	Emission coefficient
/// ISR	0.0 A	Recombination current
/// NR	2.0	Recombination coefficient
/// RS	0.0 ohm	Series resistance per unit area
/// Please refer to the SPICE documentation and the diode.py file for the others.
///
/// TANH(x)-shaped switch model
///
/// General syntax:
/// There are two possible syntax:
/// .model SW <model_id> VT=<float> VH=<float> RON=<float> ROFF=<float>
/// .model SW <model_id> VON=<float> VOFF=<float> RON=<float> ROFF=<float>
///
/// This model implements a voltage-controlled switch where the transition is modeled with tanh(x)
/// .
/// Hysteresis is supported through the parameter VH. When set, the two thresholds become VT+VH and VT-VH (distance 2*VH!).
/// When VON and VOFF are specified instead of VT and VH, the latter two are set from the former according to the relationships:
/// VT = (VON-VOFF)/2 + VOFF
/// VH = 1e-3*VT
/// Parameters and default values:
/// Parameter	Default value	Description	Restrictions
/// VT	0 V	Threshold voltage
/// VH	0 V	Hysteresis voltage	Must be positive
/// RON	1 ohm	ON-state resistance	Must be non-zero
/// ROFF	1/gmin	OFF-state resistance	Must be non-zero
///
/// Analyses
///
/// Operating point (.OP)
///
/// General syntax:
/// .op [guess=<ic_label>]
///
/// This analysis tries to find a DC solution through a pseudo Newton Rhapson (NR) iteration method. Notice that a non-linear circuit may have zero, a discrete number or infinite OPs.
/// Which one is found depends on the circuit and on the initial guess supplied to the method. The program has a built in method that tries to generate a “smart” initial guess to speed up convergence. When that fails, or is disabled from command line (see –help), the initial guess is set to all zeros.
/// The user may supply a better guess, if known. This can be done adding a .ic directive somewhere in the netlist file and setting guess=<ic_label> where <ic_label> matches the .ic’s name=<ic_label>.
/// The t = 0 value is automatically added as DC value to every time-variant independent source without a explicit DC value.
///
/// DC analysis (.DC)
///
/// General syntax:
/// .DC src=<src_name> start=<float> stop=<float> step=<float> type=<lin/log>
///
/// Performs a DC sweep (repeated OP analysis with the value of a voltage or current source changing at every iteration).
/// Parameters:
/// src: the id of the source to be swept (V12, Ibias...).
/// Only independent current and voltage sources.
/// start and stop: sweep start and stop values.
/// type: either lin or log
/// step: sets the value of the source from an iteration (k)
///  to the next (k+1)
/// :
/// if type=log, S(k+1)=S(k)⋅step
/// if type=lin, S(k+1)=S(k)+step
///
/// Transient analysis (.TRAN)
///
/// General syntax:
/// .TRAN TSTEP=<float> TSTOP=<float> [TSTART=<float>  UIC=0/1/2/3 [IC_LABEL=<string>] METHOD=<string>]
///
/// Performs a transient analysis from tstart (which defaults to 0) to tstop, using the step provided as initial step and the method specified (if any, otherwise defaults to implicit Euler).
/// Parameters:
/// tstart: the starting point, defaults to zero.
/// tstep: this is the initial step. By default, the program will try to adjust it to keep the estimate error within bounds.
/// tstop: Stop time.
/// UIC (Use Initial Conditions): This is used to specify the state of the circuit at time t = tstart. Available values are 0, 1, 2 or 3.
/// uic=0: all node voltages and currents through v/h/e/sources will be assumed to be zero at t = tstart
/// uic=1: the status at `t = tstart is the last result from a OP analysis.
/// uic=2: the status at t=tstart is the last result from a OP analysis on which are set the values of currents through inductors and voltages on capacitors specified in their ic. This is done very roughly, checking is recommended.
/// uic=3: Load a user supplied ic. This requires a .ic directive somewhere in the netlist and a .ic‘s name and ic_label must match.
/// method: the integration method to be used in transient analysis. Built-in methods are: implicit_euler, trap, gear2, gear3, gear4, gear5 and gear6. Defaults to trap. May be overridden by the value specified on the command line with the option: -t METHOD or --tran-method=METHOD.
/// High order methods are slower per iteration, but they often can afford a longer step with comparable error, hence they are actually faster in many cases.
/// If a transient analysis stops because of a step size too small, use a low order method (ie/trap) and set --t-max-nr to a high value (eg 1000).
///
/// AC analysis (.AC)
///
/// General syntax:
/// Either:
/// .AC <lin/log> <npoints> <start> <stop>
/// or:
/// .AC start=<float> stop=<float> nsteps=<integer> sweep_type=<lin/log>
///
/// Performs an AC analysis.
/// If the circuit is non-linear, a successful Operating Point (OP) is needed to linearize the circuit.
/// The sweep type is by default (and currently unchangeable) logarithmic.
/// Parameters:
/// start: the starting frequency of the sweep, in Hz.
/// stop: the final angular frequency, in Hz.
/// nsteps: the number of steps to be executed.
/// sweep_type: a parameter that can be set to LOG or LIN (the default), selecting a logarithmic or a linear frequency sweep.
/// Examples:
/// .ac lin 1 320 320
/// .ac sweep_type=lin start=320 stop=320 nsteps=1
/// Periodic Steady State (.PSS)
/// .PSS period=<float> [points=<int> step=<float> method=<string> autonomous=<bool>]
/// This analysis tries to find the periodic steady state (PSS) solution of the circuit.
/// Parameters:
/// period: the period of the solution. To be specified only in not autonomous circuits (which are somehow clocked).
/// points: How many time points to use to discretize the solution. If step is set, this is automatically computed.
/// step: Time step on the period. If points is set, this is automatically computed.
/// method: the PSS algorithm to be employed. Options are: shooting (default) and brute-force.
/// autonomous: self-explanatory boolean. If set to True, currently the simulator halts, because autonomous circuits are not supported, yet.
///
/// Pole-Zero analysis (.PZ)
/// The PZ analysis computes the poles (and optionally the zeros) of a circuit.
///
/// General syntax:
/// It can be specified with any of the following equivalent syntaxes:
/// .PZ [OUTPUT=<V(node1,node2)> SOURCE=<string> ZEROS=<bool> SHIFT=<float>]
/// or
/// .PZ [V(<node1>,<node2>) <SOURCE> <ZEROS=1> <SHIFT=0>]
///
/// Internally, it is implemented through the modification-decomposition (MD) method, which is based on finding the eigenvalues of the Time Constant Matrix (TCM).
/// All the following parameters are optional and only needed for zero calculation.
/// Parameters:
/// output: the circuit output voltage, in the form of <V(node1,node2)>. Notice the lack of space in between nodes and comma.
/// source: the part_id of the input source.
/// zeros: boolean, calculate the zeros as well. If output and source are set, then this is automatically set to 1 (true).
/// shift initial frequency shift for calculation of the singularities. Optional. In a network that has zeros in the origin, this may be set to some non-zero value since the beginning.
///
/// Symbolic small-signal (.SYMBOLIC)
/// Performs a small-signal analysis of the circuit, optionally including AC elements.
///
/// General syntax:
/// .symbolic [tf=<source_id> ac=<boolean>]
///
/// tf: If the source ID is specified, the transfer functions from the source to each of the variables in the circuit are calculated. From them, low-frequency gain, poles and zeros are extracted.
/// ac: If set to True, capacitors and inductors will be included. Defaults to False, to speed up the solutions.
/// In the results, the imaginary unit is shown as I, the angular frequency as w.
/// We rely on the Sympy library for the low-level symbolic computations. The library is under active development and might have trouble (or take a long time) with medium-big or tricky netlists. Improvements are on their way, in the meanwhile, consider simplifying complex netlists, if solving is an issue.
///
/// Post-processing
///
/// .Plot
/// Plot the results from simulation to video.
///
/// General syntax:
/// .plot <simulation_type> [variable1 variable2 ... ]
///
/// Parameters:
/// simulation_type: which simulation will have the data plotted. Currently the available options are tran, pss, ac and dc.
/// variable1, variable2: the signals to be plotted.
/// They may be:
/// a voltage, syntax V(<node>), to plot the voltage at the specified node, or V(<node2>, <node1>), to plot the difference of the node voltages. E.g. V(in) or V(2,1).
/// a current, syntax I(<source name>), e.g. I(V2) or I(Vsupply)
/// Plotting is possible only if matplotlib is available.
///
/// .Four
/// Perform a Fourier analysis over the latest transient data.
///
/// General syntax:
/// .FOUR <freq> var1 <var2 var3 ...>
///
/// The Fourier analysis is performed over the interval which is decided as follows:
/// The data should be taken from the end of the simulation, so that if there is any build-up or stabilization process, the Fourier analysis is not affected (or less affected) by it.
/// At least 1 period of the fundamental has to be used.
/// Not more than 50% of the total simulation time should be used, if possible.
/// Respecting the above, as much data as possible should be used, as it leads to more accurate results.
/// An algorithm selects the data for the Fourier transform from the data from the last transient analysis, then the data are re-sampled with a fixed time step, using a quadratic interpolation scheme.
/// A rectangular window is employed and the Fourier components are calculated using 10 frequency bins, ie 0
/// , f
/// , 2f
///  …
///  9f
/// .
/// This post-processing function prints its results to the standard output.
/// Parameters:
/// freq: the fundamental frequency, in Hz.
/// var1, var2 ... : the signals to execute the FOUR analysis on. Each signal is treated independently.
/// They may be:
/// a voltage, syntax V(<node>), e.g. V(in) or V(2,1).
/// a current, syntax I(<source name>), e.g. I(V2) or I(Vsupply)
/// Example:
/// .FOUR 100K V(n1) I(V2)
///
/// .FFT
/// FFT analysis of the time evolution of a variable.
///
/// General syntax:
///  .FFT <variable> [START=<float> STOP=<float> NP=<int>
/// + FORMAT=<string> WINDOW=<string> ALFA=<float>
/// + FREQ=<float> FMIN=<float> FMAX=<float>]
/// This post-processing analysis is a more flexible and complete version of the .FOUR statement.
/// The analysis uses a variable, user-selectable amount of time data, re-sampled with a fixed time step using quadratic interpolation, with a customizable windowing applied.
/// The time interval is specified through the start and stop parameters, if they are not set, all the available data is used. For compatibility, the simulator accepts as synonyms of start and stop the parameters from and to.
/// The function behaves differently whether the parameter freq is specified or not:
/// If the fundamental frequency freq (f
///  in the following) is specified, the analysis will perform an harmonic analysis, much like a .FOUR statement, considering only the DC component and the harmonics of f
///  from the first up to the 9th (ie f
/// , 2f
/// , 3f
///  …
///  9f
/// ).
/// If freq is left unspecified, a standard FFT analysis is performed, starting from f=0
/// , to a frequency fmax=1/(2TTOTnp)
/// , where TTOT
///  is the total length of the considered data in seconds and np
///  is the number of points in the FTT, set through the np parameter to this analysis.
/// The output data is printed to a file having a file name identical to the output file as specified with the -o switch at the invocation of the simulator, with an extension .lis appended.
/// Parameters:
/// variable: the identifier of a variable. Eg. 'V(n1)' or 'I(VS)'.
/// freq: The fundamental frequency, in Hertz. If it is specified, the output will be limited to the harmonics of this frequency. The Total Harmonic Distortion (THD) evaluation will also be enabled.
/// start: The first time instant to be considered for the transient analysis. If unspecified, it will be the beginning of the transient simulation.
/// from: Alternative specification of the start parameter.
/// stop: Last time instant to be considered for the FFT analysis. If unspecified, it will be the end time of the transient simulation.
/// to: Alternative specification of the stop parameter.
/// np: A power of two that specifies how many points should be used when computing the FFT. If it is set to a value that is not a power of 2, it will be rounded up to the nearest power of 2. It defaults to 1024.
/// window: The windowing type. The following values are available:
/// ‘RECT’ for a rectangular window, equivalent to no window at all.
/// ‘BART’, for a Bartlett window.
/// ‘HANN’, for a Hanning window.
/// ‘HAMM’ for a Hamming window.
/// ‘BLACK’ for a Blackman window.
/// ‘HARRIS’ for a Blackman-Harris window.
/// ‘GAUSS’ for a Gaussian window.
/// ‘KAISER’ for a Kaiser-Bessel window.
/// The default is the rectangular window.
/// alpha: The sigma
///  for a Gaussian window or the beta
///  for a Kaiser window. Defaults to 3 and is ignored if a window different from Gaussian or Kaiser is selected.
/// fmin: Suppress all data below this frequency, expressed in Hz. The suppressed data is neither returned nor used to compute the THD (if it is computed at all). The DC component is always preserved. Defaults to: return and use all data.
/// fmax: The dual to fmin, discard data above fmax and also do not use it if computing the THD. Expressed in Hz, defaults to infinity.
/// Example:
/// .FFT V(n1,n2) NP=1024 START=0.2u STOP=1.5u WINDOW=HANN
///
/// Other directives
/// End
/// General syntax:
/// .end
///
/// Force the parser to stop reading the netlist. Everything after this line is disregarded.
///
/// Ends
///
/// General syntax:
/// .ends
///
/// Closes a subcircuit block.
///
/// Ic
/// Set an Initial Condition for circuit analysis.
///
/// General syntax:
/// .ic name=<ic_label> [v(<node>)=<value> i(<element_name>)=<value> ... ]
///
/// This allows the specification of a state of a circuit. Every node voltage or current (through appropriate elements) may be specified. If not set, it will be set to 0. Notice that setting an inappropriate or inconsistent IC will create convergence problems.
/// Example:
/// .ic name=oscillate1 V(1)=10 V(nOUT)=2 I(VTEST)=5m
/// To use an IC directive in a transient analysis, set ‘UIC=3‘ and ‘IC_LABEL=<ic_label>‘.
///
/// Include
///
/// General syntax:
/// .include <filename>
///
/// Include a file. It’s equivalent to copy & paste the contents of the file to the bottom of the netlist.
///
/// Subckt
///
/// General syntax:
/// .subckt <subckt_label> [node1 node2 ... ]
///
/// Subcircuits are netlist block that may be called anywhere in the circuit using a subckt call. They can have other .subckt calls within - but beware of recursively calling the same subcircuit!
/// They can hold other directives, but the placement of the directive doesn’t change its meaning (i.e. if you add an .op line in the subcircuit or outside of it it’s the same).
/// They can’t be nested and have to be ended by a .ends directive.
pub mod circuit_library_peginator;
pub mod gds_library;
pub mod lef_library;

use typestate::typestate;

#[typestate]
pub mod builder {
    use super::gds_library::LGdsLibrary;
    use super::lef_library::LLefLibrary;
    use crate::circuit::graph::LCircuit;
    use crate::llhd::module::LLHDModule;

    #[derive(Debug)]
    #[automaton]
    pub struct TechnologyFlow {
        lef: LLefLibrary,
        circuit: LCircuit,
        gds: LGdsLibrary,
        module: LLHDModule,
    }

    #[state]
    pub struct Abstract;
    #[state]
    pub struct Analog;
    #[state]
    pub struct Physical;
    #[state]
    pub struct Bound;

    pub trait Abstract {
        fn unbound_library() -> Abstract;
        fn load_lef(self, library_lef: LLefLibrary) -> Analog;
    }

    pub trait Analog {
        fn construct_circuit(self) -> Physical;
    }

    pub trait Physical {
        fn load_gds(self, library_gds: LGdsLibrary) -> Bound;
    }

    pub trait Bound {
        fn bind_units(self);
    }

    impl AbstractState for TechnologyFlow<Abstract> {
        fn unbound_library() -> TechnologyFlow<Abstract> {
            Self {
                lef: LLefLibrary::default(),
                circuit: LCircuit::default(),
                gds: LGdsLibrary::default(),
                module: LLHDModule::default(),
                state: Abstract,
            }
        }

        fn load_lef(self, lef: LLefLibrary) -> TechnologyFlow<Analog> {
            TechnologyFlow::<Analog> {
                lef,
                circuit: self.circuit,
                gds: self.gds,
                module: self.module,
                state: Analog,
            }
        }
    }

    impl AnalogState for TechnologyFlow<Analog> {
        fn construct_circuit(self) -> TechnologyFlow<Physical> {
            todo!()
        }
    }

    impl PhysicalState for TechnologyFlow<Physical> {
        fn load_gds(self, _gds: LGdsLibrary) -> TechnologyFlow<Bound> {
            todo!()
        }
    }

    impl BoundState for TechnologyFlow<Bound> {
        fn bind_units(self) {
            todo!()
        }
    }
}
