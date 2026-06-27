import {
  assertValidComposition,
  createDomainManifestIndex,
  createLocalKitRegistry,
  createNativeHostComposition
} from "../native-host-domain-kits.mjs";

const targets = ["android-gles", "quest-openxr", "headless", "all"];

for (const target of targets) {
  const composition = assertValidComposition(target);

  console.log(`\n${target}`);
  console.log("- valid:", composition.validation.ok);
  console.log("- kit count:", composition.kits.length);
  console.log("- install order:", composition.summary.installOrder.join(" -> "));
}

const registry = createLocalKitRegistry();
const manifestIndex = createDomainManifestIndex();

console.log("\nregistry summary");
console.log(JSON.stringify(registry.summary(), null, 2));

console.log("\nmanifest index preview");
console.log(JSON.stringify({
  schema: manifestIndex.schema,
  version: manifestIndex.version,
  kitCount: manifestIndex.kits.length,
  firstKit: manifestIndex.kits[0]?.id,
  lastKit: manifestIndex.kits.at(-1)?.id
}, null, 2));

const android = createNativeHostComposition("android-gles");
const quest = createNativeHostComposition("quest-openxr");
const headless = createNativeHostComposition("headless");

console.log("\ncontract checks");
console.log(JSON.stringify({
  androidProvidesAndroidLifecycle: android.validation.provided.includes("n:host.android.lifecycle"),
  questProvidesOpenXrSession: quest.validation.provided.includes("n:host.openxr.session"),
  headlessProvidesBuildLogs: headless.validation.provided.includes("n:build.logs")
}, null, 2));
