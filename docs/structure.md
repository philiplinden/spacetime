# Disclaimer

1. The code in this repository is intended for analytical use for a specific
   problem. It is not meant to be a general-purpose library.
2. This project and its code are evolving as my personal understanding of
   modeling and coding techniques on the way to discovering answers to questions
   about the nature of a possible decentralized PNT service for the Moon.

# Code Organization

This project contains Python implementations and Rust implementations.

## Python
Preliminary explorations were done in Python using the Mesa framework and the
Python package from nyx-space. You can find some iPy notebooks and Mesa models
that showcase elements of these packages, along with code reference thereof.

* [Project Mesa](https://github.com/projectmesa/mesa)
* [Nyx Space](https://github.com/nyx-space/nyx)

## Rust

The Rust implementation of this model is built around an Entity-Component-System
(ECS) architecture to make the most of the tools and ethos behind Rust and core
crates like Bevy. This approach also aligns well with the intent of the model.

An ECS architecture allows for a modular and scalable design, making it easier
to manage the simulation's complexity. Components store data, and systems
operate on that data, while resources hold shared data needed across systems.
This separation of concerns and data-driven approach is a key characteristic of
ECS.

1. **Components:**
    - **Clock Component (`Clock`):**
        - Represents individual clocks in the simulation.
        - Contains parameters such as speed, local gravity, clock time, and PTP
          clock information.
        - Includes a `Radio` component for radio communication parameters.
    - **Quartz Oscillator Component (`QuartzOscillator`):**
        - Represents properties of a quartz oscillator associated with a clock.
        - Includes parameters like frequency offset, aging rate, and temperature
          sensitivity.
    - **Time Resource (`TimeResource`):**
        - A resource to keep track of the time elapsed between frames.
    - **TUI Stats Component (`TuiStats`):**
        - Holds information related to TUI (Text-based User Interface) stats for
          display.
    - **PtpClock Component (`PtpClock`):**
        - Represents PTP-related information, including offset and delay.
    - **PtpMessage Component (`PtpMessage`):**
        - Placeholder component for PTP message.
    - **Radio Component (`Radio`):**
        - Represents radio communication parameters such as transmitter power,
          receiver sensitivity, and field of view.
2. **Systems:**
    - **Time Dilation System (`TimeDilationSystem`):**
        - Calculates the time dilation factor for each clock based on speed,
          local gravity, and oscillator parameters.
        - Updates the clock time accordingly.
    - **Simulate Clock System (`SimulateClockSystem`):**
        - Simulates the behavior of each clock, updating parameters like speed,
          local gravity, and aging.
        - Handles the aging of quartz oscillators.
    - **PtpSyncSystem (`PtpSyncSystem`):**
        - Handles PTP synchronization logic, updating clock offset and delay
          based on PTP messages.
    - **Radio Communication System (`RadioCommunicationSystem`):**
        - Simulates radio communication logic, determining entities within the
          field of view and sending messages.
    - **Update TUI System (`UpdateTuiSystem`):**
        - Updates TUI stats based on simulation data.
        - Prints the stats to the terminal for visualization.
    - **PtpSyncSenderSystem (`PtpSyncSenderSystem`):**
        - Sends PTP messages to synchronize clocks.
    - **Radio Communication Sender System (`RadioCommunicationSenderSystem`):**
        - Simulates sending radio messages to entities within the field of view.
3. **Initialization:**
    - In the Bevy app, entities are created with the relevant components, such
      as `Clock`, `QuartzOscillator`, `PtpClock`, and `Radio`.
    - The ECS systems are added to the app to process and update entities during
      each frame.
4. **Interaction:**
    - Components hold the state of entities (e.g., clock parameters, oscillator
      properties, PTP information, radio communication parameters).
    - Systems process and update entities based on their components and the
      logic defined in each system.

This ECS architecture allows for a modular and scalable design, making it easier
to manage the simulation's complexity, including PTP synchronization and radio
communication. Components store data, and systems operate on that data, while
resources hold shared data needed across systems.
