import assert from "node:assert/strict";

import {
  assertValidComposition,
  createDomainManifestIndex,
  createLocalKitRegistry,
  createNativeHostComposition,
  localNativeHostDomainKits,
  targetProfiles,
  validateComposition
} from "../native-host-domain-kits.mjs";

const registry = createLocalKitRegistry();

assert.equal(localNativeHostDomainKits.length, 14, "expected all local native host domain kits to exist");
assert.equal(registry.summary().kitCount, 14, "registry summary should count all local kits");

for (const candidate of localNativeHostDomainKits) {
  assert.ok(candidate.id.endsWith("-domain-kit"), `${candidate.id} should use domain kit naming`);
  assert.ok(candidate.domain, `${candidate.id} should declare a domain`);
  assert.ok(candidate.scope, `${candidate.id} should declare a scope`);
  assert.equal(candidate.extendsBase, "DomainServiceKit", `${candidate.id} should extend the DSK contract`);
  assert.ok(Array.isArray(candidate.provides) && candidate.provides.length > 0, `${candidate.id} should provide capability tokens`);
  assert.ok(Array.isArray(candidate.requires), `${candidate.id} should declare requires array`);
  assert.ok(Array.isArray(candidate.commands), `${candidate.id} should declare commands`);
  assert.ok(Array.isArray(candidate.events), `${candidate.id} should declare events`);
  assert.ok(Array.isArray(candidate.descriptors), `${candidate.id} should declare descriptors`);
  assert.ok(candidate.snapshotPolicy, `${candidate.id} should declare snapshot policy`);
  assert.ok(candidate.resetPolicy, `${candidate.id} should declare reset policy`);
}

const requiredTargets = ["android-gles", "quest-openxr", "headless", "all"];
for (const target of requiredTargets) {
  assert.ok(targetProfiles[target], `missing target profile ${target}`);
  const composition = assertValidComposition(target);
  assert.equal(composition.validation.ok, true, `${target} composition should validate`);
  assert.equal(composition.summary.errorCount, 0, `${target} composition should not have validation errors`);
  assert.equal(composition.summary.installOrder.length, composition.kits.length, `${target} install order should include every selected kit`);
}

const android = createNativeHostComposition("android-gles");
assert.ok(android.validation.provided.includes("n:host.android.lifecycle"));
assert.ok(android.validation.provided.includes("n:render.gles"));
assert.ok(!android.kits.some((candidate) => candidate.id === "openxr-session-domain-kit"), "android-gles should not install OpenXR session kit by default");

const quest = createNativeHostComposition("quest-openxr");
assert.ok(quest.validation.provided.includes("n:host.openxr.session"));
assert.ok(quest.validation.provided.includes("n:render.stereo"));
assert.ok(quest.validation.provided.includes("n:xr.input"));

const headless = createNativeHostComposition("headless");
assert.ok(headless.validation.provided.includes("n:host.headless"));
assert.ok(headless.validation.provided.includes("n:build.logs"));
assert.ok(headless.validation.provided.includes("n:discord.summary.payload"));

const index = createDomainManifestIndex();
assert.equal(index.schema, "nexus.local-native-host-domain-kits.v1");
assert.equal(index.kits.length, localNativeHostDomainKits.length);
assert.ok(index.kits.every((candidate) => candidate.provides.length > 0));

const invalid = validateComposition({
  kits: [registry.require("gles-render-domain-kit")],
  externalCapabilities: []
});
assert.equal(invalid.ok, false, "isolated GLES render domain should fail without host/command dependencies");
assert.ok(invalid.errors.some((error) => error.missing === "n:command.buffer.output"));

const gameplayLeakTerms = ["quest completion", "combat damage", "win/loss", "objective completion"];
for (const presentationKit of [registry.require("gles-render-domain-kit"), registry.require("stereo-render-domain-kit")]) {
  const searchable = JSON.stringify(presentationKit).toLowerCase();
  for (const term of gameplayLeakTerms) {
    assert.equal(searchable.includes(`owns ${term}`), false, `${presentationKit.id} should not own ${term}`);
  }
}

console.log("local native host domain kit smoke tests passed");
