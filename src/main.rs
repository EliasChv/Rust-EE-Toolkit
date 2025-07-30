use std::io;
use std::io::Write;
use std::f64::consts::PI;

fn main() {

   println!("Hello...");
   println!("Select an option:");
loop{ 
 
    println!("A. 4 and 5 Band Resistor Solver");
    println!("B. Phasor Calculator (Magnitude & Phase)");
    println!("C. Gain Calculator (dB and Normal)");
    println!("D. Electrical Parameter Solver (V, I, R, P)");
    println!("E. Impedance Solver");
    println!("F. Fourier Series Coefficients Calculator");
    println!("G. Reactance Calculator (Capacitors & Inductors)");
    println!("H. Energy Calculator (Capacitors & Inductors)");
    println!("I. Units Converter");
    println!("J. End Code");
    print!("Your choice: ");
    io::stdout().flush().unwrap();

    let mut choice  = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let processed_choice = choice.trim().to_uppercase();
    match processed_choice.as_str() {
        "A" => {resistor_solver(); wait_for_enter(); }
        "B" => {phasor_calculator(); wait_for_enter(); }
        "C" => {gain_calculator(); wait_for_enter(); }
        "D" => {electrical_solver(); wait_for_enter(); }
        "E" => {impedance_solver(); wait_for_enter(); }
        "F" => { fourier_solver(); wait_for_enter(); }
        "G" => { reactance_calculator(); wait_for_enter(); }
        "H" => { energy_calculator(); wait_for_enter();}
        "I" => { units_converter();  wait_for_enter();}
        "J" => {
            endtask();
            break;
        }
    
        _ => println!("Invalid option."),
    }
}
}

fn endtask() {
    println!("Goodbye...");
}

fn wait_for_enter() {
    println!("\nPress Enter to return to the main menu...");
    let mut _dummy = String::new();
    io::stdin().read_line(&mut _dummy).unwrap();
}



fn color_to_digit(c: &str) -> Option<u32> {
    match c.to_lowercase().as_str() {
        "black" => Some(0),
        "brown" => Some(1),
        "red" => Some(2),
        "orange" => Some(3),
        "yellow" => Some(4),
        "green" => Some(5),
        "blue" => Some(6),
        "violet" => Some(7),
        "grey" => Some(8),
        "white" => Some(9),
        _ => None,
    }
}

fn color_to_multiplier(c: &str) -> Option<f64> {
    match c.to_lowercase().as_str() {
        "black" => Some(1.0),
        "brown" => Some(10.0),
        "red" => Some(100.0),
        "orange" => Some(1_000.0),
        "yellow" => Some(10_000.0),
        "green" => Some(100_000.0),
        "blue" => Some(1_000_000.0),
        "violet" => Some(10_000_000.0),
        "grey" => Some(100_000_000.0),
        "white" => Some(1_000_000_000.0),
        "gold" => Some(0.1),
        "silver" => Some(0.01),
        _ => None,
    }
}

fn color_to_tolerance(c: &str) -> Option<f64> {
    match c.to_lowercase().as_str() {
        "brown" => Some(1.0),
        "red" => Some(2.0),
        "green" => Some(0.5),
        "blue" => Some(0.25),
        "violet" => Some(0.1),
        "grey" => Some(0.05),
        "gold" => Some(5.0),
        "silver" => Some(10.0),
        _ => None,
    }
}

fn resistor_solver() {
    println!("Enter number of bands (4 or 5):");
    let num_bands = read_line().trim().parse::<u32>().unwrap_or(4);
    if num_bands == 4 {
        println!("Enter 1st band color:");
        let b1 = read_line();
        println!("Enter 2nd band color:");
        let b2 = read_line();
        println!("Enter multiplier color:");
        let mul = read_line();
        println!("Enter tolerance color:");
        let tol = read_line();

        if let (Some(d1), Some(d2), Some(m), Some(t)) =
            (color_to_digit(&b1), color_to_digit(&b2), color_to_multiplier(&mul), color_to_tolerance(&tol))
        {
            let ohms = ((d1 * 10 + d2) as f64) * m;
            println!(
                "Resistance: {:.2} Ω ±{}%",
                ohms,
                t
            );
        } else {
            println!("Invalid color entered.");
        }
    } else if num_bands == 5 {
        println!("Enter 1st band color:");
        let b1 = read_line();
        println!("Enter 2nd band color:");
        let b2 = read_line();
        println!("Enter 3rd band color:");
        let b3 = read_line();
        println!("Enter multiplier color:");
        let mul = read_line();
        println!("Enter tolerance color:");
        let tol = read_line();

        if let (Some(d1), Some(d2), Some(d3), Some(m), Some(t)) = (
            color_to_digit(&b1),
            color_to_digit(&b2),
            color_to_digit(&b3),
            color_to_multiplier(&mul),
            color_to_tolerance(&tol),
        ) {
            let ohms = ((d1 * 100 + d2 * 10 + d3) as f64) * m;
            println!(
                "Resistance: {:.2} Ω ±{}%",
                ohms,
                t
            );
        } else {
            println!("Invalid color entered.");
        }
    } else {
        println!("Only 4 and 5 band resistors are supported.");
    }
}

fn phasor_calculator() {
    println!("Enter real part (Re):");
    let re = read_line().trim().parse::<f64>().unwrap_or(0.0);
    println!("Enter imaginary part (Im):");
    let im = read_line().trim().parse::<f64>().unwrap_or(0.0);
    let magnitude = (re * re + im * im).sqrt();
    let phase_rad = im.atan2(re);
    let phase_deg = phase_rad.to_degrees();
    println!("Magnitude: {:.3}", magnitude);
    println!("Phase: {:.3} degrees", phase_deg);
}

fn gain_calculator() {
    println!("Enter input voltage (Vin):");
    let vin = read_line().trim().parse::<f64>().unwrap_or(0.0);
    println!("Enter output voltage (Vout):");
    let vout = read_line().trim().parse::<f64>().unwrap_or(0.0);

    if vin == 0.0 {
        println!("Input voltage cannot be zero.");
        return;
    }
    let gain = vout / vin;
    let gain_db = 20.0 * gain.abs().log10();

    println!("Gain (normal): {:.3}", gain);
    println!("Gain (dB): {:.3} dB", gain_db);

    println!("Enter input current (Iin):");
    let iin = read_line().trim().parse::<f64>().unwrap_or(0.0);
    println!("Enter output current (Iout):");
    let iout = read_line().trim().parse::<f64>().unwrap_or(0.0);

    if iin != 0.0 {
        let gain_i = iout / iin;
        let gain_i_db = 20.0 * gain_i.abs().log10();
        println!("Current Gain (normal): {:.3}", gain_i);
        println!("Current Gain (dB): {:.3} dB", gain_i_db);
    } else {
        println!("Skipping current gain calculation (Iin is zero).");
    }

    println!("Enter input power (Pin):");
    let pin = read_line().trim().parse::<f64>().unwrap_or(0.0);
    println!("Enter output power (Pout):");
    let pout = read_line().trim().parse::<f64>().unwrap_or(0.0);

    if pin != 0.0 {
        let gain_p = pout / pin;
        let gain_p_db = 10.0 * gain_p.abs().log10();
        println!("Power Gain (normal): {:.3}", gain_p);
        println!("Power Gain (dB): {:.3} dB", gain_p_db);
    } else {
        println!("Skipping power gain calculation (Pin is zero).");
    }
}

fn electrical_solver() {
    println!("Choose parameter to solve: V, I, R, or P");
    let param = read_line().to_uppercase();

    match param.trim() {
        "V" => {
            println!("Given Current & Resistance (I/R), Power & Current (P/I), or Resistance & Power (R/P)? Enter I, P, or R:");
            let choice = read_line().to_uppercase();
            if choice.trim() == "R" {
                println!("Enter Current (amps):");
                let i = read_line().trim().parse::<f64>().unwrap_or(0.0);
                println!("Enter Resistance (ohms):");
                let r = read_line().trim().parse::<f64>().unwrap_or(0.0);
                let v = r * i;
                println!("Voltage V = {:.3} volts", v);
            } else if choice.trim() == "P" {
                println!("Enter Power (watts):");
                let p = read_line().trim().parse::<f64>().unwrap_or(0.0);
                println!("Enter Current (amps):");
                let i = read_line().trim().parse::<f64>().unwrap_or(0.0);
                if i != 0.0 {
                let v = p / i;
                println!("Voltage V = {:.3} volts", v);}
                else {
                println!("Current cannot be zero.");}
            }
             else if choice.trim() == "R" {
                println!("Enter Resistance (ohms):");
                let r = read_line().trim().parse::<f64>().unwrap_or(0.0);
                println!("Enter Power (watts):");
                let p = read_line().trim().parse::<f64>().unwrap_or(0.0);
                let v = ( r * p ).sqrt();
                println!("Voltage V = {:.3} volts", v);
            }  else {
                println!("Invalid choice.");
            }
        }
        "R" => {
            println!("Given Current & Volatge (I/V), Power & Current (P/I), or Voltage & Power (V/P)? Enter I, P, or V:");
            let choice = read_line().to_uppercase();
            if choice.trim() == "I" {
                println!("Enter Current (amps):");
                let i = read_line().trim().parse::<f64>().unwrap_or(0.0);
                println!("Enter Voltage (volts):");
                let v = read_line().trim().parse::<f64>().unwrap_or(0.0);
                if i != 0.0 {
                let r = v / i;
                println!("Resistance R = {:.3} ohms", r);}
                else {
                println!("Current cannot be zero.");}
            } else if choice.trim() == "P" {
                println!("Enter Current (amps):");
                let i = read_line().trim().parse::<f64>().unwrap_or(0.0);
                println!("Enter Power (watts):");
                let p = read_line().trim().parse::<f64>().unwrap_or(0.0);
                if i != 0.0 {
                let r = p / (i*i);
                println!("Voltage V = {:.3} ohms", r);}
                else {
                println!("Current cannot be zero.");}
            }
             else if choice.trim() == "V" {
                println!("Enter Volts (volts):");
                let v = read_line().trim().parse::<f64>().unwrap_or(0.0);
                println!("Enter Power (watts):");
                let p = read_line().trim().parse::<f64>().unwrap_or(0.0);
                if p != 0.0 {
                let r = (v*v) / p ;
                println!("Resistance R = {:.3} ohms", r);}
                else {
                println!("Power cannot be zero.");}
            }  else {
                println!("Invalid choice.");
            }
        }
        "I" => {
            println!("Given Voltage & Resistance (V/R), Power & Voltage (P/V), or Resistance & Power (R/P)? Enter V, P, or R:");
            let choice = read_line().to_uppercase();
            if choice.trim() == "V" {
                println!("Enter Voltage (volts):");
                let v = read_line().trim().parse::<f64>().unwrap_or(0.0);
                println!("Enter Resistance (ohms):");
                let r = read_line().trim().parse::<f64>().unwrap_or(0.0);
                if r != 0.0 {
                let i = v / r;
                println!("Current I = {:.3} amps", i);}
                else {
                println!("Resistance cannot be zero.");}
            } else if choice.trim() == "P" {
                println!("Enter Power (watts):");
                let p = read_line().trim().parse::<f64>().unwrap_or(0.0);
                println!("Enter Voltage (volts):");
                let v = read_line().trim().parse::<f64>().unwrap_or(0.0);
                if v != 0.0 {
                let i = p / v;
                println!("Current I = {:.3} amps", i);}
                else {
                println!("Voltage cannot be zero.");}
            }
             else if choice.trim() == "R" {
                println!("Enter Resistance (ohms):");
                let r = read_line().trim().parse::<f64>().unwrap_or(0.0);
                println!("Enter Power (watts):");
                let p = read_line().trim().parse::<f64>().unwrap_or(0.0);
                if r != 0.0 {
                let i = ( p/r ).sqrt();
                println!("Current I = {:.3} amps", i);}
                else {
                println!("Resistance cannot be zero.");}
            }  else {
                println!("Invalid choice.");
            }
        }
        "P" => {
            println!("Given Resistance & Current (R/I), Current & Voltage (I/V), or Voltage & Resistance (V/R)? Enter R, I, or V:");
            let choice = read_line().to_uppercase();
            if choice.trim() == "R" {
                println!("Enter Resistance (ohms):");
                let r = read_line().trim().parse::<f64>().unwrap_or(0.0);
                println!("Enter Current (amps):");
                let i = read_line().trim().parse::<f64>().unwrap_or(0.0);
                let p = r * i * i;
                println!("Power P = {:.3} watts", p);
            } else if choice.trim() == "I" {
                println!("Enter Current (amps):");
                let i = read_line().trim().parse::<f64>().unwrap_or(0.0);
                println!("Enter Volts (volts):");
                let v = read_line().trim().parse::<f64>().unwrap_or(0.0);
                let p = i * v;
                println!("Power P = {:.3} watts", p);
            }
             else if choice.trim() == "V" {
                println!("Enter Volts (volts):");
                let v = read_line().trim().parse::<f64>().unwrap_or(0.0);
                println!("Enter Resistance (ohms):");
                let r = read_line().trim().parse::<f64>().unwrap_or(0.0);
                if r != 0.0 {
                let p = (v*v) / r ;
                println!("Power P = {:.3} watts", p);}
                else {
                println!("Resistance cannot be zero.");}
            }  else {
                println!("Invalid choice.");
            }
        }
        _ => println!("Unsupported parameter."),
    }
}

fn impedance_solver() {
    println!("Enter resistance (R) in ohms:");
    let r = read_line().trim().parse::<f64>().unwrap_or(0.0);
    println!("Enter reactance (X) in ohms:");
    let x = read_line().trim().parse::<f64>().unwrap_or(0.0);
    let magnitude = (r * r + x * x).sqrt();
    let phase = x.atan2(r).to_degrees();
    println!("Impedance magnitude = {:.3} Ω", magnitude);
    println!("Impedance phase = {:.3} degrees", phase);
}

fn fourier_solver() {
    println!("Calculate Fourier series coefficients of a periodic function.\n");

    // Let the user choose the function type
    println!("Choose the function type:");
    println!("1. Square Wave");
    println!("2. Triangle Wave");
    println!("3. Sine Wave");

    let func_choice = read_line().trim().parse::<u32>().unwrap_or(1);
    
    println!("Enter the period (T) of the signal:");
    let period = read_line().trim().parse::<f64>().unwrap_or(2.0);

    println!("Enter the number of terms (N) to approximate the Fourier series:");
    let terms = read_line().trim().parse::<usize>().unwrap_or(10);

    // Based on the function type, calculate Fourier coefficients
    match func_choice {
        1 => square_wave_fourier(period, terms),
        2 => triangle_wave_fourier(period, terms),
        3 => sine_wave_fourier(period, terms),
        _ => println!("Invalid choice."),
    }
}

fn square_wave_fourier(period: f64, terms: usize) {
    println!("\nCalculating Fourier Series for Square Wave:");
    println!("Period (T): {}", period);
    println!("Number of terms: {}", terms);

    let mut bn = 0.0;

    for n in 1..=terms {
        // Only odd harmonics for square wave
        if n % 2 != 0 {
            bn = (4.0 / PI) * (1.0 / n as f64); // b_n for square wave
            println!("b_{} = {:.3}", n, bn);
        }
    }

    println!("\nThe Fourier series representation of the Square Wave is:");
    print!("f(t) = ");
    for n in 1..=terms {
        if n % 2 != 0 {
            print!("{:.3}sin({}t) + ", bn, n);
        }
    }
    println!("... (for more terms, increase N).");
}

fn triangle_wave_fourier(period: f64, terms: usize) {
    println!("\nCalculating Fourier Series for Triangle Wave:");
    println!("Period (T): {}", period);
    println!("Number of terms: {}", terms);

    let mut an = 0.0;

    for n in 1..=terms {
        an = -(2.0 / (PI * n as f64));
        println!("a_{} = {:.3}", n, an);     // For triangle, a_n = -2/(n*pi)
    }

    println!("\nThe Fourier series representation of the Triangle Wave is:");
    print!("f(t) = ");
    for n in 1..=terms {
        print!("{:.3}cos({}t) + ", an, n);
    }
    println!("... (for more terms, increase N).");
}

fn sine_wave_fourier(period: f64, terms: usize) {
    println!("\nCalculating Fourier Series for Sine Wave:");
    println!("Period (T): {}", period);
    println!("Number of terms: {}", terms);


    let bn = 1.0;  // for fundamental frequency

    // Display the result
    println!("\nThe Fourier series representation of the Sine Wave is:");
    println!("f(t) = {:.3}sin({}t)", bn, 1);
}


fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


fn reactance_calculator() {
    println!("\nSelect the type of reactance to calculate:");
    println!("1. Capacitive Reactance (X_C)");
    println!("2. Inductive Reactance (X_L)");

    let choice = read_line().trim().parse::<u32>().unwrap_or(1);

    match choice {
        1 => capacitive_reactance(),
        2 => inductive_reactance(),
        _ => println!("Invalid choice."),
    }
}

fn capacitive_reactance() {
    println!("\nCapacitive Reactance Calculation:");
    println!("Enter the frequency (f) in Hz:");
    let f = read_line().trim().parse::<f64>().unwrap_or(0.0);
    println!("Enter the capacitance (C) in farads:");
    let c = read_line().trim().parse::<f64>().unwrap_or(0.0);

    if f > 0.0 && c > 0.0 {
        let omega = 2.0 * std::f64::consts::PI * f;  // angular frequency (omega = 2πf)
        let xc = 1.0 / (omega * c);  // capacitive reactance formula
        println!("Capacitive Reactance (X_C) = {:.3} Ω", xc);
    } else {
        println!("Frequency and capacitance must be greater than zero.");
    }
}

fn inductive_reactance() {
    println!("\nInductive Reactance Calculation:");
    println!("Enter the frequency (f) in Hz:");
    let f = read_line().trim().parse::<f64>().unwrap_or(0.0);
    println!("Enter the inductance (L) in henries:");
    let l = read_line().trim().parse::<f64>().unwrap_or(0.0);

    if f > 0.0 && l > 0.0 {
        let omega = 2.0 * std::f64::consts::PI * f;  // angular frequency (omega = 2πf)
        let xl = omega * l;  // inductive reactance formula
        println!("Inductive Reactance (X_L) = {:.3} Ω", xl);
    } else {
        println!("Frequency and inductance must be greater than zero.");
    }
}


fn energy_calculator() {
    println!("\nSelect the component to calculate energy for:");
    println!("1. Capacitor");
    println!("2. Inductor");

    let choice = read_line().trim().parse::<u32>().unwrap_or(1);

    match choice {
        1 => capacitor_energy(),
        2 => inductor_energy(),
        _ => println!("Invalid choice."),
    }
}

fn capacitor_energy() {
    println!("\nCapacitor Energy Calculation:");
    println!("Enter the capacitance (C) in farads:");
    let c = read_line().trim().parse::<f64>().unwrap_or(0.0);
    println!("Enter the voltage (V) across the capacitor in volts:");
    let v = read_line().trim().parse::<f64>().unwrap_or(0.0);

    if c > 0.0 && v > 0.0 {
        let energy = 0.5 * c * v * v; // Energy formula for capacitor
        println!("Energy stored in the capacitor: {:.6} joules", energy);
    } else {
        println!("Capacitance and voltage must be greater than zero.");
    }
}

fn inductor_energy() {
    println!("\nInductor Energy Calculation:");
    println!("Enter the inductance (L) in henries:");
    let l = read_line().trim().parse::<f64>().unwrap_or(0.0);
    println!("Enter the current (I) through the inductor in amperes:");
    let i = read_line().trim().parse::<f64>().unwrap_or(0.0);

    if l > 0.0 && i > 0.0 {
        let energy = 0.5 * l * i * i; // Energy formula for inductor
        println!("Energy stored in the inductor: {:.6} joules", energy);
    } else {
        println!("Inductance and current must be greater than zero.");
    }
}


fn units_converter() {
    println!("\nSelect the type of unit conversion:");
    println!("1. dB to Power Ratio / Power Ratio to dB");
    println!("2. dB to Voltage Ratio / Voltage Ratio to dB");
    println!("3. Capacitance (F, uF, nF, pF)");
    println!("4. Inductance (H, mH, uH)");
    println!("5. Time (seconds, milliseconds, microseconds)");
    println!("6. Power (W, kW, mW)");
    println!("7. Frequency (Hz, kHz, MHz)");
    println!("8. Impedance (Ohms, KOhms, MOhms)");
    print!("Your choice: ");
    
    let choice = read_line().trim().parse::<u32>().unwrap_or(1);

    match choice {
        1 => db_to_power_ratio(),
        2 => db_to_voltage_ratio(),
        3 => capacitance_converter(),
        4 => inductance_converter(),
        5 => time_converter(),
        6 => power_converter(),
        7 => frequency_converter(),
        8 => impedance_converter(),
        _ => println!("Invalid choice."),
    }
}

fn db_to_power_ratio() {
    println!("\nConvert between dB and Power Ratio:");
    println!("1. dB to Power Ratio");
    println!("2. Power Ratio to dB");
    print!("Your choice: ");
    
    let choice = read_line().trim().parse::<u32>().unwrap_or(1);
    
    if choice == 1 {
        println!("Enter dB value: ");
        let db = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let power_ratio = 10f64.powf(db / 10.0);
        println!("Power Ratio = {:.3}", power_ratio);
    } else if choice == 2 {
        println!("Enter Power Ratio: ");
        let power_ratio = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let db = 10.0 * power_ratio.log(10.0);
        println!("dB = {:.3}", db);
    } else {
        println!("Invalid choice.");
    }
}

fn db_to_voltage_ratio() {
    println!("\nConvert between dB and Voltage Ratio:");
    println!("1. dB to Voltage Ratio");
    println!("2. Voltage Ratio to dB");
    print!("Your choice: ");
    
    let choice = read_line().trim().parse::<u32>().unwrap_or(1);
    
    if choice == 1 {
        println!("Enter dB value: ");
        let db = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let voltage_ratio = 10f64.powf(db / 20.0);
        println!("Voltage Ratio = {:.3}", voltage_ratio);
    } else if choice == 2 {
        println!("Enter Voltage Ratio: ");
        let voltage_ratio = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let db = 20.0 * voltage_ratio.log(10.0);
        println!("dB = {:.3}", db);
    } else {
        println!("Invalid choice.");
    }
}

fn capacitance_converter() {
    println!("\nConvert capacitance values:");
    println!("1. Farads to Microfarads");
    println!("2. Microfarads to Farads");
    println!("3. Nanofarads to Microfarads");
    println!("4. Microfarads to Nanofarads");
    print!("Your choice: ");
    
    let choice = read_line().trim().parse::<u32>().unwrap_or(1);
    
    if choice == 1 {
        println!("Enter capacitance in Farads: ");
        let farads = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let microfarads = farads * 1_000_000.0;
        println!("Capacitance = {:.3} µF", microfarads);
    } else if choice == 2 {
        println!("Enter capacitance in Microfarads: ");
        let microfarads = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let farads = microfarads / 1_000_000.0;
        println!("Capacitance = {:.6} F", farads);
    } else if choice == 3 {
        println!("Enter capacitance in Nanofarads: ");
        let nanofarads = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let microfarads = nanofarads / 1_000.0;
        println!("Capacitance = {:.3} µF", microfarads);
    } else if choice == 4 {
        println!("Enter capacitance in Microfarads: ");
        let microfarads = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let nanofarads = microfarads * 1_000.0;
        println!("Capacitance = {:.3} nF", nanofarads);
    } else {
        println!("Invalid choice.");
    }
}

fn inductance_converter() {
    println!("\nConvert inductance values:");
    println!("1. Henries to Millihenries");
    println!("2. Millihenries to Henries");
    println!("3. Microhenries to Millihenries");
    println!("4. Millihenries to Microhenries");
    print!("Your choice: ");
    
    let choice = read_line().trim().parse::<u32>().unwrap_or(1);
    
    if choice == 1 {
        println!("Enter inductance in Henries: ");
        let henries = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let millihenries = henries * 1_000.0;
        println!("Inductance = {:.3} mH", millihenries);
    } else if choice == 2 {
        println!("Enter inductance in Millihenries: ");
        let millihenries = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let henries = millihenries / 1_000.0;
        println!("Inductance = {:.6} H", henries);
    } else if choice == 3 {
        println!("Enter inductance in Microhenries: ");
        let microhenries = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let millihenries = microhenries / 1_000.0;
        println!("Inductance = {:.3} mH", millihenries);
    } else if choice == 4 {
        println!("Enter inductance in Millihenries: ");
        let millihenries = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let microhenries = millihenries * 1_000.0;
        println!("Inductance = {:.3} µH", microhenries);
    } else {
        println!("Invalid choice.");
    }
}

fn time_converter() {
    println!("\nConvert time values:");
    println!("1. Seconds to Milliseconds");
    println!("2. Milliseconds to Seconds");
    println!("3. Seconds to Microseconds");
    println!("4. Microseconds to Seconds");
    print!("Your choice: ");
    
    let choice = read_line().trim().parse::<u32>().unwrap_or(1);
    
    if choice == 1 {
        println!("Enter time in Seconds: ");
        let seconds = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let milliseconds = seconds * 1_000.0;
        println!("Time = {:.3} ms", milliseconds);
    } else if choice == 2 {
        println!("Enter time in Milliseconds: ");
        let milliseconds = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let seconds = milliseconds / 1_000.0;
        println!("Time = {:.6} s", seconds);
    } else if choice == 3 {
        println!("Enter time in Seconds: ");
        let seconds = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let microseconds = seconds * 1_000_000.0;
        println!("Time = {:.3} µs", microseconds);
    } else if choice == 4 {
        println!("Enter time in Microseconds: ");
        let microseconds = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let seconds = microseconds / 1_000_000.0;
        println!("Time = {:.6} s", seconds);
    } else {
        println!("Invalid choice.");
    }
}

fn power_converter() {
    println!("\nConvert power values:");
    println!("1. Watts to Kilowatts");
    println!("2. Kilowatts to Watts");
    println!("3. Watts to Milliwatts");
    println!("4. Milliwatts to Watts");
    print!("Your choice: ");
    
    let choice = read_line().trim().parse::<u32>().unwrap_or(1);
    
    if choice == 1 {
        println!("Enter power in Watts: ");
        let watts = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let kilowatts = watts / 1_000.0;
        println!("Power = {:.3} kW", kilowatts);
    } else if choice == 2 {
        println!("Enter power in Kilowatts: ");
        let kilowatts = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let watts = kilowatts * 1_000.0;
        println!("Power = {:.3} W", watts);
    } else if choice == 3 {
        println!("Enter power in Watts: ");
        let watts = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let milliwatts = watts * 1_000.0;
        println!("Power = {:.3} mW", milliwatts);
    } else if choice == 4 {
        println!("Enter power in Milliwatts: ");
        let milliwatts = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let watts = milliwatts / 1_000.0;
        println!("Power = {:.3} W", watts);
    } else {
        println!("Invalid choice.");
    }
}

fn frequency_converter() {
    println!("\nConvert frequency values:");
    println!("1. Hertz to Kilohertz");
    println!("2. Kilohertz to Hertz");
    println!("3. Hertz to Megahertz");
    println!("4. Megahertz to Hertz");
    println!("5. Kilohertz to Megahertz");
    println!("6. Megahertz to Kilohertz");
    print!("Your choice: ");
    
    let choice = read_line().trim().parse::<u32>().unwrap_or(1);
    
    if choice == 1 {
        println!("Enter frequency in Hertz: ");
        let hertz = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let kilohertz = hertz / 1_000.0;
        println!("Frequency = {:.3} kHz", kilohertz);
    } else if choice == 2 {
        println!("Enter frequency in Kilohertz: ");
        let kilohertz = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let hertz = kilohertz * 1_000.0;
        println!("Frequency = {:.3} Hz", hertz);
    } else if choice == 3 {
        println!("Enter frequency in Hertz: ");
        let hertz = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let megahertz = hertz / 1_000_000.0;
        println!("Frequency = {:.6} MHz", megahertz);
    } else if choice == 4 {
        println!("Enter frequency in Megahertz: ");
        let megahertz = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let hertz = megahertz * 1_000_000.0;
        println!("Frequency = {:.3} Hz", hertz);
    } else if choice == 5 {
        println!("Enter frequency in Kilohertz: ");
        let kilohertz = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let megahertz = kilohertz / 1_000.0;
        println!("Frequency = {:.3} MHz", megahertz);
    } else if choice == 6 {
        println!("Enter frequency in Megahertz: ");
        let megahertz = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let kilohertz = megahertz * 1_000.0;
        println!("Frequency = {:.3} kHz", kilohertz);
    } else {
        println!("Invalid choice.");
    }
}

fn impedance_converter() {
    println!("\nConvert impedance values:");
    println!("1. Ohms to Kilohms");
    println!("2. Kilohms to Ohms");
    println!("3. Ohms to Megohms");
    println!("4. Megohms to Ohms");
    println!("5. Kilohms to Megohms");
    println!("6. Megohms to Kilohms");
    print!("Your choice: ");
    
    let choice = read_line().trim().parse::<u32>().unwrap_or(1);
    
    if choice == 1 {
        println!("Enter impedance in Ohms: ");
        let ohms = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let kilohms = ohms / 1_000.0;
        println!("Impedance = {:.3} kΩ", kilohms);
    } else if choice == 2 {
        println!("Enter impedance in Kilohms: ");
        let kilohms = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let ohms = kilohms * 1_000.0;
        println!("Impedance = {:.3} Ω", ohms);
    } else if choice == 3 {
        println!("Enter impedance in Ohms: ");
        let ohms = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let megohms = ohms / 1_000_000.0;
        println!("Impedance = {:.6} MΩ", megohms);
    } else if choice == 4 {
        println!("Enter impedance in Megohms: ");
        let megohms = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let ohms = megohms * 1_000_000.0;
        println!("Impedance = {:.3} Ω", ohms);
    } else if choice == 5 {
        println!("Enter impedance in Kilohms: ");
        let kilohms = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let megohms = kilohms / 1_000.0;
        println!("Impedance = {:.3} MΩ", megohms);
    } else if choice == 6 {
        println!("Enter impedance in Megohms: ");
        let megohms = read_line().trim().parse::<f64>().unwrap_or(0.0);
        let kilohms = megohms * 1_000.0;
        println!("Impedance = {:.3} kΩ", kilohms);
    } else {
        println!("Invalid choice.");
    }
}
// End test