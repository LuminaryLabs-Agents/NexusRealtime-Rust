export const LOCAL_NATIVE_HOST_DOMAIN_KIT_VERSION = "0.1.0";

export const EXTERNAL_NATIVE_HOST_CAPABILITIES = Object.freeze([
  "n:runtime.engine",
  "n:nexus.core.runtime",
  "n:nexus.gameplay.domains",
  "n:ci.runner",
  "n:platform.android",
  "n:platform.openxr",
  "n:platform.desktop",
  "n:platform.headless"
]);

function kit(definition) {
  return Object.freeze({
    version: LOCAL_NATIVE_HOST_DOMAIN_KIT_VERSION,
    extendsBase: "DomainServiceKit",
    ownsLoop: false,
    snapshotPolicy: "serializable",
    resetPolicy: "engine-reset-aware",
    ownedState: [],
    commands: [],
    events: [],
    descriptors: [],
    lifecycle: [],
    diagnostics: [],
    ...definition
  });
}

export const localNativeHostDomainKits = Object.freeze([
  kit({
    id: "project-bundle-domain-kit",
    domain: "project-bundle",
    scope: "host-support-domain",
    provides: [
      "n:project.bundle",
      "n:project.loading",
      "n:project.validation"
    ],
    requires: [
      "n:runtime.engine"
    ],
    ownedState: [
      "ProjectBundleState",
      "ProjectSchemaState",
      "ProjectValidationLedger"
    ],
    commands: [
      "project.bundle.load.request",
      "project.bundle.validate.request",
      "project.bundle.reload.request"
    ],
    events: [
      "project.bundle.loaded",
      "project.bundle.validated",
      "project.bundle.rejected"
    ],
    descriptors: [
      "ProjectBundleDescriptor",
      "ProjectValidationReport"
    ],
    diagnostics: [
      "loadedProjectId",
      "entrySequence",
      "kitManifestPath",
      "validationErrors"
    ],
    promotionTarget: "NexusRealtime core host/project-loader contract",
    notes: "Keeps project parsing and validation out of platform hosts."
  }),

  kit({
    id: "adaptive-host-profile-domain-kit",
    domain: "adaptive-host-profile",
    scope: "host-support-domain",
    provides: [
      "n:host.profile.selection",
      "n:host.capability.negotiation",
      "n:host.profile.validation"
    ],
    requires: [
      "n:project.bundle",
      "n:runtime.engine"
    ],
    ownedState: [
      "AdaptiveHostProfileState",
      "HostCapabilityState"
    ],
    commands: [
      "host.profile.select.request",
      "host.capabilities.register.request",
      "host.profile.validate.request"
    ],
    events: [
      "host.profile.selected",
      "host.capabilities.registered",
      "host.profile.rejected"
    ],
    descriptors: [
      "AdaptiveHostProfileDescriptor",
      "HostCapabilityDescriptor"
    ],
    diagnostics: [
      "selectedProfileId",
      "requiredCapabilities",
      "missingCapabilities"
    ],
    promotionTarget: "NexusRealtime core host/adaptive profile contract",
    notes: "Lets Android, OpenXR, desktop, and headless hosts share one profile language."
  }),

  kit({
    id: "command-buffer-domain-kit",
    domain: "command-buffer",
    scope: "host-support-domain",
    provides: [
      "n:command.buffer.output",
      "n:command.buffer.schema",
      "n:host.presentation.commands"
    ],
    requires: [
      "n:runtime.engine"
    ],
    ownedState: [
      "CommandBufferSchemaState",
      "CommandBufferFrameState",
      "CommandBufferValidationLedger"
    ],
    commands: [
      "command.buffer.emit.request",
      "command.buffer.validate.request",
      "command.buffer.clear.request"
    ],
    events: [
      "command.buffer.emitted",
      "command.buffer.validated",
      "command.buffer.rejected"
    ],
    descriptors: [
      "CommandBufferDescriptor",
      "CommandSchemaDescriptor",
      "PresentationCommandDescriptor"
    ],
    diagnostics: [
      "frame",
      "commandCount",
      "schemaVersion",
      "unknownCommands"
    ],
    promotionTarget: "NexusRealtime core command-buffer contract and Rust nexus-command-buffer crate",
    notes: "The primary bridge from Nexus runtime/sequence output to native hosts and renderers."
  }),

  kit({
    id: "input-routing-domain-kit",
    domain: "input-routing",
    scope: "host-support-domain",
    provides: [
      "n:input.routing",
      "n:host.input.normalization",
      "n:core.input.packet"
    ],
    requires: [
      "n:host.profile.selection",
      "n:command.buffer.output",
      "n:nexus.core.runtime"
    ],
    ownedState: [
      "InputRoutingState",
      "InputPacketState",
      "InputFrameLedger"
    ],
    commands: [
      "input.packet.route.request",
      "input.source.register.request",
      "input.packet.replay.request"
    ],
    events: [
      "input.packet.routed",
      "input.source.registered",
      "input.packet.rejected"
    ],
    descriptors: [
      "InputRoutingDescriptor",
      "InputPacketDescriptor"
    ],
    diagnostics: [
      "lastInputFrameId",
      "activeSources",
      "rejectedInputs"
    ],
    promotionTarget: "NexusRealtime core host input contract",
    notes: "Input requests actions. Gameplay DSKs validate meaning."
  }),

  kit({
    id: "diagnostics-trace-domain-kit",
    domain: "diagnostics-trace",
    scope: "host-support-domain",
    provides: [
      "n:diagnostics.trace",
      "n:runtime.health.report",
      "n:domain.debug.ledger"
    ],
    requires: [
      "n:runtime.engine",
      "n:command.buffer.output"
    ],
    ownedState: [
      "DiagnosticsTraceState",
      "RuntimeHealthState",
      "DomainDebugLedger"
    ],
    commands: [
      "diagnostics.trace.record.request",
      "diagnostics.health.sample.request",
      "diagnostics.trace.flush.request"
    ],
    events: [
      "diagnostics.trace.recorded",
      "diagnostics.health.sampled",
      "diagnostics.trace.flushed"
    ],
    descriptors: [
      "RuntimeHealthDescriptor",
      "DomainTraceDescriptor",
      "FrameDiagnosticsDescriptor"
    ],
    diagnostics: [
      "frame",
      "domainEvents",
      "hostEvents",
      "warnings",
      "errors"
    ],
    promotionTarget: "NexusRealtime core diagnostics/surface contract",
    notes: "Every meaningful host action should be traceable without making the renderer own truth."
  }),

  kit({
    id: "android-lifecycle-domain-kit",
    domain: "android-lifecycle",
    scope: "adapter-domain",
    provides: [
      "n:host.android.lifecycle",
      "n:host.jni.bridge",
      "n:host.android.asset.loading"
    ],
    requires: [
      "n:host.kernel",
      "n:project.bundle",
      "n:command.buffer.output",
      "n:platform.android"
    ],
    ownedState: [
      "AndroidLifecycleState",
      "JniBridgeState",
      "AndroidAssetMountState"
    ],
    commands: [
      "android.lifecycle.init.request",
      "android.lifecycle.resume.request",
      "android.lifecycle.pause.request",
      "android.lifecycle.shutdown.request",
      "android.asset.load.request"
    ],
    events: [
      "android.lifecycle.initialized",
      "android.lifecycle.resumed",
      "android.lifecycle.paused",
      "android.lifecycle.shutdown",
      "android.command.rejected"
    ],
    descriptors: [
      "AndroidLifecycleDescriptor",
      "JniStatusDescriptor",
      "AndroidAssetDescriptor"
    ],
    lifecycle: [
      "init",
      "resume",
      "pause",
      "shutdown",
      "tick"
    ],
    diagnostics: [
      "jniStatus",
      "androidStage",
      "assetLoadErrors"
    ],
    snapshotPolicy: "host-trace",
    resetPolicy: "lifecycle-aware",
    promotionTarget: "Rust nexus-android-bridge and later a ProtoKit adapter manifest",
    notes: "Owns Android/JNI lifecycle only. It must not own gameplay rules."
  }),

  kit({
    id: "openxr-session-domain-kit",
    domain: "openxr-session",
    scope: "adapter-domain",
    provides: [
      "n:host.openxr.session",
      "n:xr.frame.loop",
      "n:xr.reference.spaces",
      "n:xr.projection.layer"
    ],
    requires: [
      "n:host.kernel",
      "n:host.profile.selection",
      "n:command.buffer.output",
      "n:platform.openxr"
    ],
    ownedState: [
      "OpenXrLifecycleState",
      "OpenXrSessionState",
      "OpenXrFrameLoopState",
      "OpenXrSwapchainState"
    ],
    commands: [
      "openxr.session.start.request",
      "openxr.session.stop.request",
      "openxr.frame.wait.request",
      "openxr.frame.submit.request",
      "openxr.space.locate.request"
    ],
    events: [
      "openxr.session.started",
      "openxr.session.stopped",
      "openxr.frame.started",
      "openxr.frame.submitted",
      "openxr.command.rejected"
    ],
    descriptors: [
      "OpenXrSessionDescriptor",
      "OpenXrFrameDescriptor",
      "OpenXrProjectionLayerDescriptor"
    ],
    lifecycle: [
      "loaderReady",
      "instanceReady",
      "sessionReady",
      "swapchainsReady",
      "frameLoopReady"
    ],
    diagnostics: [
      "bootStage",
      "eyeCount",
      "swapchainState",
      "frameTiming"
    ],
    snapshotPolicy: "host-trace",
    resetPolicy: "lifecycle-aware",
    promotionTarget: "Rust nexus-openxr-host and later native OpenXR adapter contract",
    notes: "OpenXR session/frame loop domain. It presents descriptors and input, not gameplay truth."
  }),

  kit({
    id: "xr-input-domain-kit",
    domain: "xr-input",
    scope: "adapter-domain",
    provides: [
      "n:xr.input",
      "n:xr.hand.packet",
      "n:xr.controller.packet"
    ],
    requires: [
      "n:host.openxr.session",
      "n:input.routing"
    ],
    ownedState: [
      "XrInputState",
      "XrHandPacketState",
      "XrControllerPacketState"
    ],
    commands: [
      "xr.input.sync.request",
      "xr.input.synthetic.request",
      "xr.input.replay.request"
    ],
    events: [
      "xr.input.synced",
      "xr.input.synthetic.created",
      "xr.input.rejected"
    ],
    descriptors: [
      "XrInputFrameDescriptor",
      "XrHandDescriptor",
      "XrControllerDescriptor"
    ],
    diagnostics: [
      "leftHandActive",
      "rightHandActive",
      "controllerProfile",
      "inputFrameId"
    ],
    snapshotPolicy: "host-trace",
    resetPolicy: "frame-aware",
    promotionTarget: "Rust nexus-xr-input and later ProtoKit XR input adapter",
    notes: "Normalizes XR input into input packets. Gameplay meaning belongs to gameplay DSKs."
  }),

  kit({
    id: "xr-interaction-domain-kit",
    domain: "xr-interaction",
    scope: "detector-domain",
    provides: [
      "n:xr.interaction.candidates",
      "n:xr.grab.detector",
      "n:xr.pointer.detector"
    ],
    requires: [
      "n:xr.input",
      "n:input.routing"
    ],
    ownedState: [
      "XrInteractionCandidateState",
      "XrGrabDetectorState",
      "XrPointerDetectorState"
    ],
    commands: [
      "xr.interaction.scan.request",
      "xr.grab.detect.request",
      "xr.pointer.detect.request"
    ],
    events: [
      "xr.interaction.candidate.detected",
      "xr.grab.candidate.detected",
      "xr.pointer.candidate.detected",
      "xr.interaction.rejected"
    ],
    descriptors: [
      "XrInteractionCandidateDescriptor",
      "XrGrabCandidateDescriptor"
    ],
    diagnostics: [
      "candidateCount",
      "nearestCandidateId",
      "detectorMode"
    ],
    snapshotPolicy: "host-trace",
    resetPolicy: "frame-aware",
    promotionTarget: "Rust nexus-xr-interaction as detector; gameplay DSK consumes emitted candidates",
    notes: "This is intentionally a detector domain, not a gameplay grab/throw rule owner."
  }),

  kit({
    id: "gles-render-domain-kit",
    domain: "gles-render",
    scope: "presentation-domain",
    provides: [
      "n:render.gles",
      "n:render.command.presentation",
      "n:render.surface.android"
    ],
    requires: [
      "n:command.buffer.output",
      "n:host.android.lifecycle"
    ],
    ownedState: [
      "GlesRenderState",
      "GlesResourceState",
      "GlesSurfaceState"
    ],
    commands: [
      "gles.render.init.request",
      "gles.render.present.request",
      "gles.render.resize.request",
      "gles.render.shutdown.request"
    ],
    events: [
      "gles.render.initialized",
      "gles.render.presented",
      "gles.render.resized",
      "gles.render.rejected"
    ],
    descriptors: [
      "GlesRenderDescriptor",
      "GlesMaterialDescriptor",
      "GlesSurfaceDescriptor"
    ],
    lifecycle: [
      "initSurface",
      "resizeSurface",
      "presentFrame",
      "disposeSurface"
    ],
    diagnostics: [
      "surfaceReady",
      "drawCalls",
      "commandCount",
      "resourceWarnings"
    ],
    snapshotPolicy: "host-trace",
    resetPolicy: "surface-aware",
    promotionTarget: "Rust nexus-render-gles as renderer implementation and adapter manifest",
    notes: "Renderer reads command buffers/descriptors only. It must not decide gameplay outcomes."
  }),

  kit({
    id: "stereo-render-domain-kit",
    domain: "stereo-render",
    scope: "presentation-domain",
    provides: [
      "n:render.stereo",
      "n:render.openxr.projection",
      "n:render.eye.views"
    ],
    requires: [
      "n:command.buffer.output",
      "n:host.openxr.session"
    ],
    ownedState: [
      "StereoRenderState",
      "EyeViewState",
      "ProjectionLayerState"
    ],
    commands: [
      "stereo.render.init.request",
      "stereo.render.present.request",
      "stereo.eye.views.update.request",
      "stereo.render.shutdown.request"
    ],
    events: [
      "stereo.render.initialized",
      "stereo.render.presented",
      "stereo.eye.views.updated",
      "stereo.render.rejected"
    ],
    descriptors: [
      "StereoRenderDescriptor",
      "EyeViewDescriptor",
      "ProjectionLayerDescriptor"
    ],
    lifecycle: [
      "initProjectionLayer",
      "locateEyeViews",
      "presentStereoFrame",
      "disposeProjectionLayer"
    ],
    diagnostics: [
      "eyeCount",
      "projectionLayerReady",
      "drawCallsPerEye",
      "frameTiming"
    ],
    snapshotPolicy: "host-trace",
    resetPolicy: "session-aware",
    promotionTarget: "Rust stereo renderer contract and OpenXR renderer adapter",
    notes: "Stereo presentation domain. It consumes OpenXR/session descriptors and command buffers."
  }),

  kit({
    id: "headless-replay-domain-kit",
    domain: "headless-replay",
    scope: "host-domain",
    provides: [
      "n:host.headless",
      "n:runtime.replay",
      "n:ci.validation"
    ],
    requires: [
      "n:host.kernel",
      "n:project.bundle",
      "n:command.buffer.output",
      "n:platform.headless"
    ],
    ownedState: [
      "HeadlessReplayState",
      "ReplayInputState",
      "CiValidationState"
    ],
    commands: [
      "headless.run.request",
      "headless.replay.request",
      "headless.validate.request"
    ],
    events: [
      "headless.run.completed",
      "headless.replay.completed",
      "headless.validation.completed",
      "headless.command.rejected"
    ],
    descriptors: [
      "HeadlessRunDescriptor",
      "ReplayTraceDescriptor",
      "CiValidationDescriptor"
    ],
    lifecycle: [
      "loadFixture",
      "runTicks",
      "validateOutput",
      "writeReport"
    ],
    diagnostics: [
      "tickCount",
      "determinismHash",
      "commandBufferHash",
      "validationFailures"
    ],
    snapshotPolicy: "serializable",
    resetPolicy: "run-aware",
    promotionTarget: "Rust headless host crate or nexus-host submodule",
    notes: "Proves the same host kernel can run without Android, OpenXR, or GLES."
  }),

  kit({
    id: "build-artifact-log-domain-kit",
    domain: "build-artifact-log",
    scope: "artifact-domain",
    provides: [
      "n:build.logs",
      "n:build.artifact.manifest",
      "n:discord.summary.payload"
    ],
    requires: [
      "n:ci.runner",
      "n:runtime.health.report"
    ],
    ownedState: [
      "BuildRunState",
      "BuildArtifactManifestState",
      "BuildLogLedger"
    ],
    commands: [
      "build.log.capture.request",
      "build.artifact.record.request",
      "build.report.write.request",
      "discord.summary.create.request"
    ],
    events: [
      "build.log.captured",
      "build.artifact.recorded",
      "build.report.written",
      "discord.summary.created",
      "build.command.rejected"
    ],
    descriptors: [
      "BuildReportDescriptor",
      "ArtifactManifestDescriptor",
      "DiscordSummaryDescriptor"
    ],
    diagnostics: [
      "runId",
      "commitSha",
      "platformResults",
      "artifactHashes",
      "logPaths"
    ],
    snapshotPolicy: "repo-ledger",
    resetPolicy: "run-aware",
    promotionTarget: "Build pipeline support domain, likely remains repo-local or CI adapter",
    notes: "Writes full build evidence to repo logs while keeping Discord compact."
  }),

  kit({
    id: "host-kernel-domain-kit",
    domain: "host-kernel",
    scope: "host-kernel-domain",
    provides: [
      "n:host.kernel",
      "n:host.domain.registry",
      "n:host.tick.orchestration"
    ],
    requires: [
      "n:runtime.engine",
      "n:nexus.core.runtime",
      "n:project.bundle",
      "n:host.profile.selection",
      "n:input.routing",
      "n:command.buffer.output",
      "n:diagnostics.trace"
    ],
    ownedState: [
      "HostKernelState",
      "HostDomainRegistryState",
      "HostFrameState",
      "HostKernelDiagnosticsState"
    ],
    commands: [
      "host.kernel.start.request",
      "host.kernel.tick.request",
      "host.kernel.pause.request",
      "host.kernel.resume.request",
      "host.kernel.stop.request",
      "host.domain.install.request"
    ],
    events: [
      "host.kernel.started",
      "host.kernel.ticked",
      "host.kernel.paused",
      "host.kernel.resumed",
      "host.kernel.stopped",
      "host.domain.installed",
      "host.kernel.command.rejected"
    ],
    descriptors: [
      "HostKernelDescriptor",
      "HostDomainRegistryDescriptor",
      "HostFrameDescriptor",
      "HostKernelDiagnosticsDescriptor"
    ],
    lifecycle: [
      "installDomains",
      "start",
      "tick",
      "pause",
      "resume",
      "stop"
    ],
    diagnostics: [
      "installedDomains",
      "activeHostProfile",
      "frame",
      "lastCommandBufferSummary",
      "lastHostPresentReport"
    ],
    snapshotPolicy: "host-trace",
    resetPolicy: "kernel-aware",
    promotionTarget: "Rust nexus-host core kernel and later promoted host kernel contract",
    notes: "Central composer. It should orchestrate domains, not own gameplay rules."
  })
]);

export const defaultExternalCapabilities = Object.freeze(new Set(EXTERNAL_NATIVE_HOST_CAPABILITIES));

export const targetProfiles = Object.freeze({
  "android-gles": Object.freeze({
    id: "android-gles",
    externalCapabilities: [
      "n:runtime.engine",
      "n:nexus.core.runtime",
      "n:nexus.gameplay.domains",
      "n:platform.android"
    ],
    include: [
      "project-bundle-domain-kit",
      "adaptive-host-profile-domain-kit",
      "command-buffer-domain-kit",
      "input-routing-domain-kit",
      "diagnostics-trace-domain-kit",
      "host-kernel-domain-kit",
      "android-lifecycle-domain-kit",
      "gles-render-domain-kit"
    ]
  }),
  "quest-openxr": Object.freeze({
    id: "quest-openxr",
    externalCapabilities: [
      "n:runtime.engine",
      "n:nexus.core.runtime",
      "n:nexus.gameplay.domains",
      "n:platform.openxr"
    ],
    include: [
      "project-bundle-domain-kit",
      "adaptive-host-profile-domain-kit",
      "command-buffer-domain-kit",
      "input-routing-domain-kit",
      "diagnostics-trace-domain-kit",
      "host-kernel-domain-kit",
      "openxr-session-domain-kit",
      "xr-input-domain-kit",
      "xr-interaction-domain-kit",
      "stereo-render-domain-kit"
    ]
  }),
  headless: Object.freeze({
    id: "headless",
    externalCapabilities: [
      "n:runtime.engine",
      "n:nexus.core.runtime",
      "n:nexus.gameplay.domains",
      "n:platform.headless",
      "n:ci.runner"
    ],
    include: [
      "project-bundle-domain-kit",
      "adaptive-host-profile-domain-kit",
      "command-buffer-domain-kit",
      "input-routing-domain-kit",
      "diagnostics-trace-domain-kit",
      "host-kernel-domain-kit",
      "headless-replay-domain-kit",
      "build-artifact-log-domain-kit"
    ]
  }),
  all: Object.freeze({
    id: "all",
    externalCapabilities: EXTERNAL_NATIVE_HOST_CAPABILITIES,
    include: localNativeHostDomainKits.map((candidate) => candidate.id)
  })
});

export function createLocalKitRegistry(kits = localNativeHostDomainKits) {
  const byId = new Map();
  const providedBy = new Map();

  for (const candidate of kits) {
    if (byId.has(candidate.id)) {
      throw new Error(`Duplicate local kit id: ${candidate.id}`);
    }
    byId.set(candidate.id, candidate);

    for (const token of candidate.provides ?? []) {
      const providers = providedBy.get(token) ?? [];
      providers.push(candidate.id);
      providedBy.set(token, providers);
    }
  }

  return Object.freeze({
    kits: Object.freeze([...kits]),
    byId,
    providedBy,
    get(id) {
      return byId.get(id) ?? null;
    },
    require(id) {
      const candidate = byId.get(id);
      if (!candidate) throw new Error(`Unknown local kit id: ${id}`);
      return candidate;
    },
    providersFor(token) {
      return Object.freeze([...(providedBy.get(token) ?? [])]);
    },
    summary() {
      return {
        version: LOCAL_NATIVE_HOST_DOMAIN_KIT_VERSION,
        kitCount: kits.length,
        providedTokenCount: providedBy.size,
        kits: kits.map((candidate) => ({
          id: candidate.id,
          domain: candidate.domain,
          scope: candidate.scope,
          provides: candidate.provides,
          requires: candidate.requires
        }))
      };
    }
  });
}

export function validateComposition({ kits, externalCapabilities = [] }) {
  const registry = createLocalKitRegistry(kits);
  const provided = new Set(externalCapabilities);
  const errors = [];

  for (const candidate of kits) {
    for (const token of candidate.provides ?? []) {
      provided.add(token);
    }
  }

  for (const candidate of kits) {
    for (const token of candidate.requires ?? []) {
      if (!provided.has(token)) {
        errors.push({
          kitId: candidate.id,
          missing: token,
          message: `${candidate.id} requires ${token}, but no selected kit or external capability provides it.`
        });
      }
    }
  }

  return {
    ok: errors.length === 0,
    errors,
    provided: [...provided].sort(),
    installOrder: getInstallOrder({ kits, externalCapabilities, registry })
  };
}

export function createNativeHostComposition(target = "all") {
  const profile = targetProfiles[target];
  if (!profile) {
    throw new Error(`Unknown local native host target profile: ${target}`);
  }

  const registry = createLocalKitRegistry();
  const selected = profile.include.map((id) => registry.require(id));
  const validation = validateComposition({
    kits: selected,
    externalCapabilities: profile.externalCapabilities
  });

  return Object.freeze({
    target: profile.id,
    externalCapabilities: profile.externalCapabilities,
    kits: Object.freeze(selected),
    validation,
    summary: summarizeComposition({
      target: profile.id,
      kits: selected,
      validation
    })
  });
}

export function summarizeComposition({ target, kits, validation }) {
  const byScope = kits.reduce((acc, candidate) => {
    acc[candidate.scope] = (acc[candidate.scope] ?? 0) + 1;
    return acc;
  }, {});

  return {
    target,
    kitCount: kits.length,
    valid: validation.ok,
    errorCount: validation.errors.length,
    byScope,
    installOrder: validation.installOrder.map((candidate) => candidate.id)
  };
}

function getInstallOrder({ kits, externalCapabilities = [], registry = createLocalKitRegistry(kits) }) {
  const selectedIds = new Set(kits.map((candidate) => candidate.id));
  const external = new Set(externalCapabilities);
  const installed = new Set();
  const ordered = [];
  let changed = true;

  while (ordered.length < kits.length && changed) {
    changed = false;

    for (const candidate of kits) {
      if (installed.has(candidate.id)) continue;

      const ready = (candidate.requires ?? []).every((token) => {
        if (external.has(token)) return true;
        const providers = registry.providersFor(token).filter((id) => selectedIds.has(id));
        return providers.length === 0 || providers.some((id) => installed.has(id));
      });

      if (ready) {
        ordered.push(candidate);
        installed.add(candidate.id);
        changed = true;
      }
    }
  }

  if (ordered.length !== kits.length) {
    const remaining = kits.filter((candidate) => !installed.has(candidate.id)).map((candidate) => candidate.id);
    throw new Error(`Could not resolve local kit install order. Remaining: ${remaining.join(", ")}`);
  }

  return ordered;
}

export function createDomainManifestIndex(kits = localNativeHostDomainKits) {
  return {
    schema: "nexus.local-native-host-domain-kits.v1",
    version: LOCAL_NATIVE_HOST_DOMAIN_KIT_VERSION,
    generatedFor: "NexusRealtime-Rust local staging",
    kits: kits.map((candidate) => ({
      id: candidate.id,
      domain: candidate.domain,
      scope: candidate.scope,
      extendsBase: candidate.extendsBase,
      provides: candidate.provides,
      requires: candidate.requires,
      ownsLoop: candidate.ownsLoop,
      snapshotPolicy: candidate.snapshotPolicy,
      resetPolicy: candidate.resetPolicy,
      ownedState: candidate.ownedState,
      commands: candidate.commands,
      events: candidate.events,
      descriptors: candidate.descriptors,
      lifecycle: candidate.lifecycle,
      diagnostics: candidate.diagnostics,
      promotionTarget: candidate.promotionTarget,
      notes: candidate.notes
    }))
  };
}

export function assertValidComposition(target = "all") {
  const composition = createNativeHostComposition(target);
  if (!composition.validation.ok) {
    throw new Error(
      `Invalid ${target} composition:\n` +
      composition.validation.errors.map((error) => `- ${error.message}`).join("\n")
    );
  }
  return composition;
}
