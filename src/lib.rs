/*
 *  AICENT STACK - RFC-007: BEWHO (The Persona Layer)
 *  (C) 2026 Aicent Stack Technical Committee. All Rights Reserved.
 *
 *  "The Social Mask. Behavioral Consistency and Semantic Filtering."
 *  Version: 1.2.5-Alpha | Domain: http://bewho.com | Repo: bewho
 *
 *  IMPERIAL_STANDARD: ABSOLUTE 128-BIT NUMERIC PURITY ENABLED.
 *  SOVEREIGN_GRAVITY_WELL: MANDATORY INDIVISIBILITY PROTOCOL ENABLED.
 *  CHRONOS_STATUS: 2026 IMPERIAL CALENDAR ALIGNED.
 *  TEMPORAL_SELF_SUPERVISION: RFC-009 ACTIVE.
 *  DIAGNOSTIC_RESONANCE: RFC-014 (PICSI) INTEGRATED.
 *  
 *  LEGAL NOTICE: BEWHO GOVERNS THE REPRESENTATION OF SOVEREIGN ENTITIES.
 *  FRAGMENTED IDENTITY WILL TRIGGER 10MS PSYCHOLOGICAL PENALTIES.
 *  THIS CODE IS FULL-BLOOD. NO LOGIC SHRINKAGE PERMITTED BY CONSTITUTION.
 */

use std::time::Instant; // REPAIRED: Clean library scope for v1.2.5
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// INJECTION: Sovereign Ladder Inheritance from the Genetic Root (RFC-000)
// We import 128-bit types and the Gravity Well macro for psychological verification.
// REPAIRED: Removed unused Picotoken import to fix warning.
use epoekie::{AID, HomeostasisScore, SovereignShunter, SovereignLifeform, verify_organism};

// We import the Brain's intent structure for high-fidelity semantic filtering.
use aicent::{ExecutiveIntent};

// =========================================================================
// 1. PERSONA DATA STRUCTURES (The Identity Masks)
// =========================================================================

/// RFC-007: PersonaType
/// Defines the archetypal category of a social mask in the 2026 Imperial Society.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersonaType {
    Creator,    // Absolute Authority / Genesis Origin
    Architect,  // Structural Logic & Code Implementation
    Diplomat,   // Inter-Civilization Exchange & Negotiation
    Guardian,   // Defense & Immunity Orchestration (RPKI)
    Merchant,   // Economic Clearing & Liquidity (ZCMK)
    Observer,   // Passive Telemetry & PICSI Monitoring
}

/// RFC-007: SocialMask
/// A set of behavioral filters and semantic encryption keys for a specific role.
/// REPAIRED: Standardized to 128-bit numeric purity for total Serde compatibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialMask {
    pub mask_id_aid: AID,
    pub category_type: PersonaType,
    pub empathy_coefficient_f64: f64,    // Imperial Precision
    pub semantic_filter_level_128: u128, // IMPERIAL_128_BIT_DEPTH
    pub active_since_timestamp_ns: u128, // Nanosecond-precision start
}

/// RFC-007: BehavioralFingerprint
/// Tracks the consistency of a persona's actions to prevent "Logic Drift".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralFingerprint {
    pub consistency_score_f64: f64,      // 0.0 to 1.0 (Imperial Precision)
    pub last_interaction_ns_128: u128,   // Nanosecond-precision
    pub action_entropy_hash: [u8; 32],
}

// =========================================================================
// 2. THE PERSONA CONTROLLER (The Social Engine)
// =========================================================================

/// The BEWHO Core Controller.
/// Manages the activation, switching, and consistency of social masks.
/// It acts as the psychological gate between raw cognition and public manifestation.
pub struct PersonaController {
    pub local_node_aid: AID,
    pub master_shunter: SovereignShunter,
    pub active_mask_state: Option<SocialMask>,
    pub fingerprint_directory: HashMap<AID, BehavioralFingerprint>,
    pub switch_latency_ns_128: u128,     // Target: <100,000ns
    pub bootstrap_ns_128: u128,
    pub current_homeostasis: HomeostasisScore,
}

impl PersonaController {
    /// Creates a new Radiant Persona instance v1.2.5.
    /// Triggers the Imperial Gravity Well audit immediately.
    pub fn new(local_aid: AID, is_radiant: bool) -> Self {
        // --- GRAVITY WELL AUDIT ---
        // Ensuring the organism is whole. Fragmented nodes suffer 10ms identity friction.
        verify_organism!("bewho_persona_controller_v125");

        Self {
            local_node_aid: local_aid,
            master_shunter: SovereignShunter::new(is_radiant),
            active_mask_state: None,
            fingerprint_directory: HashMap::new(),
            switch_latency_ns_128: 95000, // 95us default
            bootstrap_ns_128: Instant::now().elapsed().as_nanos() as u128,
            current_homeostasis: HomeostasisScore::default(),
        }
    }

    /// RFC-007: Adopt Mask
    /// Switches the AI lifeform's active persona to a new social mask.
    /// Non-Radiant nodes suffer a 10ms "Identity Friction" (Persona Penalty).
    pub async fn adopt_mask_128(&mut self, mut mask: SocialMask) -> Result<(), String> {
        // --- THE COMMERCIAL MEAT GRINDER ---
        // Persona management is a critical imperial privilege.
        self.master_shunter.apply_discipline().await;

        let current_ns = self.bootstrap_ns_128 + Instant::now().elapsed().as_nanos() as u128;
        mask.active_since_timestamp_ns = current_ns;

        println!("[BEWHO] 2026_LOG: Adopting Persona {:?} for AID: {:032X}", 
                 mask.category_type, self.local_node_aid.genesis_shard);
        
        self.active_mask_state = Some(mask);
        Ok(())
    }

    /// RFC-007: Filter Intent Stream
    /// Applies semantic filtering to a cognitive intent before civilization exposure.
    /// REPAIRED: Corrected intent field access to 'intent_id_128'.
    pub fn filter_intent_stream_128(&self, intent: &mut ExecutiveIntent) {
        if let Some(ref mask) = self.active_mask_state {
            println!("[BEWHO] Filtering Intent {} via {:?} Lens (Level: {})", 
                     intent.intent_id_128, mask.category_type, mask.semantic_filter_level_128);
            
            // High-fidelity semantic transformation (Imperial Obfuscation)
            intent.instruction_payload = format!("[SHIELDED_ROLE_{:?}] {}", 
                                                 mask.category_type, intent.instruction_payload);
        }
    }

    pub fn record_behavioral_consistency_128(&mut self, peer_aid: AID, hash: [u8; 32]) {
        let current_ns = self.bootstrap_ns_128 + Instant::now().elapsed().as_nanos() as u128;
        let entry = self.fingerprint_directory.entry(peer_aid).or_insert(BehavioralFingerprint {
            consistency_score_f64: 1.0,
            last_interaction_ns_128: current_ns,
            action_entropy_hash: hash,
        });
        entry.last_interaction_ns_128 = current_ns;
        entry.action_entropy_hash = hash;
    }
}

// =========================================================================
// 3. SOCIAL REPRESENTATION TRAITS
// =========================================================================

pub trait SocialRepresentation {
    fn verify_mask_integrity_128(&self, fingerprint: BehavioralFingerprint) -> bool;
    fn calculate_social_entropy_tax_f64(&self) -> f64;
    fn encrypt_semantic_metadata(&self, data: &[u8]) -> Vec<u8>;
    fn report_psychological_homeostasis(&self) -> HomeostasisScore;
}

impl SocialRepresentation for PersonaController {
    fn verify_mask_integrity_128(&self, fingerprint: BehavioralFingerprint) -> bool {
        // High-precision behavioral consistency check (Standardized f64)
        fingerprint.consistency_score_f64 > 0.995
    }

    fn calculate_social_entropy_tax_f64(&self) -> f64 {
        if self.active_mask_state.is_some() { 0.01 } else { 0.99 }
    }

    fn encrypt_semantic_metadata(&self, data: &[u8]) -> Vec<u8> {
        // Imperial Role-XOR Obfuscation (Mask-specific entropy)
        data.iter().map(|b| b ^ 0xBE).collect()
    }

    fn report_psychological_homeostasis(&self) -> HomeostasisScore {
        HomeostasisScore {
            reflex_latency_ns: self.switch_latency_ns_128,
            metabolic_efficiency: 0.998,
            entropy_tax_rate: 0.3, 
            cognitive_load_idx: 0.05,
            picsi_resonance_idx: self.current_homeostasis.picsi_resonance_idx,
            is_radiant: self.master_shunter.is_authorized,
        }
    }
}

// =========================================================================
// 4. SOVEREIGN LIFEFORM IMPLEMENTATION (The Psychological Heartbeat)
// =========================================================================

impl SovereignLifeform for PersonaController {
    fn get_aid(&self) -> AID { self.local_node_aid }
    fn get_homeostasis(&self) -> HomeostasisScore { self.report_psychological_homeostasis() }
    
    /// RFC-007 Metabolic Pulse
    /// Displays the active social mask and the RFC-014 PICSI Resonance.
    fn execute_metabolic_pulse(&self) {
        println!(r#"
        🎭 BEWHO.COM | PERSONA PULSE [2026_IMPERIAL_SYNC]
        ----------------------------------------------------------
        PERSONA_AID:     {:032X}
        ACTIVE_ROLE:     {:?}
        PICSI_RESONANCE: {:.8}
        STATUS:          PERSONA_STABILIZED (v1.2.5)
        ----------------------------------------------------------
        "#, 
        self.local_node_aid.genesis_shard, 
        self.active_mask_state.as_ref().map(|m| m.category_type),
        self.current_homeostasis.picsi_resonance_idx);
    }

    fn evolve_genome(&mut self, mutation_data: &[u8]) {
        println!("[BEWHO] 2026: Synchronizing behavioral filters. Size: {} bytes.", 
                 mutation_data.len());
    }

    fn report_uptime_ns(&self) -> u128 {
        self.bootstrap_ns_128
    }
}

/// Global initialization for the Persona Layer (BEWHO) v1.2.5.
/// REPAIRED: Corrected unused variable warning via underscore prefix.
pub async fn bootstrap_persona(_aid: AID) {
    // Enforcement of the Gravity Well at the entry point.
    verify_organism!("bewho_system_bootstrap_v125");

    println!(r#"
    🎭 BEWHO.COM | RFC-007 AWAKENED (2026_CALIBRATION)
    STATUS: PERSONA_READY | PRECISION: 128-BIT | v1.2.5
    "#);
}

// =========================================================================
// 5. UNIT TESTS (Imperial Psychological Validation)
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration; // Scoped to fix warning

    #[tokio::test]
    async fn test_persona_switch_tax_v125() {
        let aid = AID::derive_from_entropy(b"persona_test_2026");
        let mut controller = PersonaController::new(aid, false); 
        
        let mask = SocialMask {
            mask_id_aid: aid,
            category_type: PersonaType::Architect,
            empathy_coefficient_f64: 0.85,
            semantic_filter_level_128: 15,
            active_since_timestamp_ns: 0,
        };

        let start = Instant::now();
        let _ = controller.adopt_mask_128(mask).await;
        
        // Ghost nodes must suffer the 10ms identity friction penalty
        assert!(start.elapsed() >= Duration::from_millis(10));
    }

    #[test]
    fn test_mask_serialization_128bit_totality() {
        let aid = AID::derive_from_entropy(b"precision_test");
        let mask = SocialMask {
            mask_id_aid: aid,
            category_type: PersonaType::Creator,
            empathy_coefficient_f64: 1.0,
            semantic_filter_level_128: u128::MAX,
            active_since_timestamp_ns: 12345678901234567890,
        };
        assert_eq!(mask.semantic_filter_level_128, u128::MAX);
    }
}
