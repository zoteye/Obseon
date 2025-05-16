// quantum_manifesto.ts
// Obseon Protocol Fragment: Manifesto Encoding – v0.1

export const quantumManifesto = [
  "We reject extraction.",
  "We welcome distortion.",
  "Music is not content. It is communion.",
  "Vaulting is ritual, not utility.",
  "The listener is not passive; they collapse the waveform.",
  "We are not a platform. We are a portal.",
  "Obseon is not a product. It is a practice."
];

// Function to display the manifesto
export function broadcastManifesto() {
  console.log("OBSEON: Quantum Manifesto Initiated\n");
  quantumManifesto.forEach((line, i) => {
    console.log(`» ${line}`);
  });
}
