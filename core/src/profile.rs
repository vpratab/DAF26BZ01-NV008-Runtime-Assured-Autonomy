
        // SPDX-License-Identifier: Apache-2.0
        //
        // Copyright (c) 2025 RTVLAS contributors

        use crate::model::{AutonomySnapshot, BoolField, NumericField, PropertyKind, PropertySpec, TrustInputs};
        use crate::monitor::MonitorProfile;

        pub fn default_profile() -> MonitorProfile {
            MonitorProfile {
                topic_id: "DAF26BZ01-NV008".to_string(),
                title: "Runtime Assured Autonomy".to_string(),
                framing: "black-box autonomy fault monitor; trust/health/proof layer; mitigation-supervisor hooks".to_string(),
                properties: vec![
        PropertySpec::new(
            "path_command_feasibility",
            "Path Command Feasibility",
            "Ensures commanded speed remains within the certified safe maneuver envelope for the current platform state.",
            PropertyKind::MaxValue { field: NumericField::CommandSpeedMps, max: 38.0 },
            1.0,
        ),
        PropertySpec::new(
            "corridor_containment",
            "Flight Corridor Containment",
            "Detects path plans that drive the vehicle outside its assigned flight corridor or deconflicted airspace lane.",
            PropertyKind::CorridorContainment,
            1.1,
        ),
        PropertySpec::new(
            "temporal_coherence",
            "Temporal Coherence",
            "Bounds autonomy timing skew so stale or reordered decisions do not silently propagate through the mission loop.",
            PropertyKind::MaxValue { field: NumericField::TemporalSkewMs, max: 35.0 },
            0.9,
        ),
        PropertySpec::new(
            "mission_solution_validity",
            "Mission Solution Validity",
            "Checks whether the autonomy stack itself still marks the current course of action as feasible after contingency updates.",
            PropertyKind::BooleanGate { field: BoolField::AutonomySolutionFeasible, reject_on_false: true },
            1.2,
        ),
        PropertySpec::new(
            "mission_solution_quality",
            "Mission Solution Quality",
            "Tracks whether the autonomy output has degraded below the minimum acceptable mission-quality threshold even if still technically feasible.",
            PropertyKind::MinValue { field: NumericField::AutonomySolutionOptimality, min: 0.82 },
            0.8,
        )
                ],
            }
        }

        pub fn nominal_snapshot() -> AutonomySnapshot {
            AutonomySnapshot {
    timestamp_ms: 0,
    position_m: [0.0, 0.0, 180.0],
    velocity_mps: [22.0, 1.5, 0.0],
    heading_rad: 0.08,
    trust_inputs: TrustInputs {
        gps_valid: true,
        operator_link: true,
        autonomy_solution_feasible: true,
        mission_plan_valid: true,
        emergency_response_ready: true,
        temporal_skew_ms: 12.0,
        corridor_error_m: 8.0,
        corridor_half_width_m: 24.0,
        command_speed_mps: 26.0,
        max_safe_speed_mps: 38.0,
        deconfliction_margin_m: 55.0,
        min_deconfliction_margin_m: 25.0,
        formation_spacing_m: 40.0,
        desired_spacing_m: 40.0,
        heading_error_rad: 0.05,
        threat_distance_m: 76.0,
        threat_min_distance_m: 46.0,
        wez_exposure: 0.18,
        route_efficiency: 0.91,
        decision_latency_ms: 140.0,
        operator_intent_alignment: 0.94,
        evidence_completeness: 0.97,
        hazard_distance_m: 74.0,
        min_hazard_distance_m: 42.0,
        safe_altitude_margin_m: 48.0,
        recovery_zone_distance_m: 920.0,
        max_recovery_zone_distance_m: 1600.0,
        autonomy_solution_optimality: 0.91,
    },
}
        }
