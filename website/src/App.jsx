import { ArrowRight, BookOpen, Github, MessageSquare, Sparkles } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";

const baseUrl = import.meta.env.BASE_URL || "/";
const withBase = (path = "") => `${baseUrl}${path}`;
const repoDocsBase = "https://github.com/Harkirat155/Aether/blob/main/docs";
const logoPath = withBase("aether-logo.svg");

const docs = [
  ["Language Specification", `${repoDocsBase}/specs/AetherLanguageSpecification.md`],
  ["Compiler Architecture", `${repoDocsBase}/specs/AetherCompilerArchitectureSpec.md`],
  ["Implementation Roadmap", `${repoDocsBase}/project/AetherImplementationRoadmap.md`],
  ["Execution Backlog", `${repoDocsBase}/project/AetherExecutionBacklog.md`],
  ["Conformance Plan", `${repoDocsBase}/project/AetherConformanceAndQualityPlan.md`],
  ["Docs Index", `${repoDocsBase}/project/AetherDocsIndex.md`]
];

const directives = [
  "Treat agency as a first-class language concern.",
  "Require explicit effects and capabilities for meaningful actions.",
  "Guarantee deterministic replay for trust and operability.",
  "Align autonomous action with policy and human governance."
];

export default function App() {
  return (
    <div className="min-h-screen">
      <header className="border-b border-border/70">
        <div className="mx-auto flex w-[min(1100px,92vw)] items-center justify-between py-5">
          <a href={baseUrl} className="inline-flex items-center gap-2 text-lg font-semibold tracking-tight">
            <img src={logoPath} alt="Aether logo" className="size-7" />
            <span>Aether</span>
          </a>
          <div className="hidden items-center gap-6 text-sm text-muted-foreground md:flex">
            <a href="#philosophy" className="hover:text-foreground">Philosophy</a>
            <a href="#directives" className="hover:text-foreground">Directives</a>
            <a href="#participate" className="hover:text-foreground">Participate</a>
            <a href="https://github.com/Harkirat155/Aether" className="hover:text-foreground">GitHub</a>
          </div>
        </div>
      </header>

      <main>
        <section className="mx-auto w-[min(1100px,92vw)] py-20">
          <Badge className="mb-4" variant="secondary">Frontier Programming Language</Badge>
          <h1 className="max-w-4xl text-4xl font-semibold tracking-tight md:text-6xl">
            Aether is the language for building trustworthy autonomous coding systems.
          </h1>
          <p className="mt-6 max-w-3xl text-lg text-muted-foreground">
            It is purpose-built for agentic software: explicit reasoning constructs, strict capability and effect boundaries,
            deterministic replay, and policy-governed execution for real-world deployment.
          </p>
          <div className="mt-8 flex flex-wrap gap-3">
            <a href={`${repoDocsBase}/specs/AetherLanguageSpecification.md`}><Button size="lg">Read the Language Spec <ArrowRight className="size-4" /></Button></a>
            <a href="https://github.com/Harkirat155/Aether"><Button variant="outline" size="lg"><Github className="size-4" /> View Repository</Button></a>
          </div>
        </section>

        <section id="philosophy" className="border-y border-border/70 bg-card/20 py-16">
          <div className="mx-auto grid w-[min(1100px,92vw)] gap-6 md:grid-cols-3">
            <Card>
              <CardHeader><CardTitle>Need</CardTitle></CardHeader>
              <CardContent className="text-muted-foreground">Existing stacks bolt agent behaviors onto tools without language-level guarantees for safety, traceability, and governance.</CardContent>
            </Card>
            <Card>
              <CardHeader><CardTitle>Philosophy</CardTitle></CardHeader>
              <CardContent className="text-muted-foreground">Aether treats autonomous reasoning and action as core semantics, not framework conventions.</CardContent>
            </Card>
            <Card>
              <CardHeader><CardTitle>Promise</CardTitle></CardHeader>
              <CardContent className="text-muted-foreground">Powerful autonomy with explicit controls: capabilities, effects, replay traces, and human gates.</CardContent>
            </Card>
          </div>
        </section>

        <section id="directives" className="mx-auto w-[min(1100px,92vw)] py-16">
          <div className="mb-8 flex items-center gap-2">
            <Sparkles className="size-5 text-primary" />
            <h2 className="text-3xl font-semibold">Aether directives</h2>
          </div>
          <div className="grid gap-4 md:grid-cols-2">
            {directives.map((directive) => (
              <Card key={directive} className="shadow-glow/10">
                <CardHeader>
                  <CardDescription>{directive}</CardDescription>
                </CardHeader>
              </Card>
            ))}
          </div>
        </section>

        <section className="border-y border-border/70 bg-card/20 py-16">
          <div className="mx-auto w-[min(1100px,92vw)]">
            <div className="mb-6 flex items-center gap-2">
              <BookOpen className="size-5 text-primary" />
              <h2 className="text-3xl font-semibold">Read the foundations</h2>
            </div>
            <div className="grid gap-3 md:grid-cols-2">
              {docs.map(([label, href]) => (
                <a key={label} href={href} className="rounded-lg border border-border bg-card px-4 py-3 text-sm hover:bg-muted/50">
                  {label}
                </a>
              ))}
            </div>
          </div>
        </section>

        <section id="participate" className="mx-auto w-[min(1100px,92vw)] py-16">
          <h2 className="text-3xl font-semibold">Participate in building Aether</h2>
          <p className="mt-4 max-w-3xl text-muted-foreground">
            This project is intentionally collaborative across language design, compiler engineering, runtime safety, and developer tooling.
            Contributions from humans and coding agents are welcome and encouraged.
          </p>
          <div className="mt-8 grid gap-4 md:grid-cols-3">
            <Card>
              <CardHeader>
                <CardTitle>Propose ideas</CardTitle>
                <CardDescription>Use GitHub Discussions for design conversations, strategy, and roadmap feedback.</CardDescription>
              </CardHeader>
              <CardContent>
                <a href="https://github.com/Harkirat155/Aether/discussions"><Button variant="secondary"><MessageSquare className="size-4" /> Open Discussions</Button></a>
              </CardContent>
            </Card>
            <Card>
              <CardHeader>
                <CardTitle>Report and request</CardTitle>
                <CardDescription>Use Issues for bugs, feature requests, and implementation tasks with invariant impact details.</CardDescription>
              </CardHeader>
              <CardContent>
                <a href="https://github.com/Harkirat155/Aether/issues"><Button variant="outline">Open Issues</Button></a>
              </CardContent>
            </Card>
            <Card>
              <CardHeader>
                <CardTitle>Contribute directly</CardTitle>
                <CardDescription>Follow contributor and agent protocols to ship PRs aligned with Aether safety and determinism goals.</CardDescription>
              </CardHeader>
              <CardContent>
                <a href="https://github.com/Harkirat155/Aether/blob/main/CONTRIBUTING.md"><Button variant="outline">Contribution Guide</Button></a>
              </CardContent>
            </Card>
          </div>
        </section>
      </main>

      <footer className="border-t border-border/70 py-8 text-sm text-muted-foreground">
        <div className="mx-auto flex w-[min(1100px,92vw)] flex-col justify-between gap-3 md:flex-row">
          <p>Built for safe, auditable, and effective autonomous software.</p>
          <div className="flex gap-4">
            <a href="https://github.com/Harkirat155/Aether" className="hover:text-foreground">GitHub</a>
            <a href="https://github.com/Harkirat155/Aether/discussions" className="hover:text-foreground">Discussions</a>
            <a href="https://github.com/Harkirat155/Aether/issues" className="hover:text-foreground">Issues</a>
          </div>
        </div>
      </footer>
    </div>
  );
}
