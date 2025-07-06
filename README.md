# Syracuse_CSE681_Software_Modeling

Graduate course at Syracuse University (CSE 681) taught by Dr. Greg Wagner. Covers software modeling techniques including GUI modeling, multithreaded/hyperthreaded systems, and software network modeling. Includes examples, tools, and project materials.

## Course Overview

This repository contains coursework, examples, and projects for CSE 681 - Software Modeling and Analysis. The course focuses on formal and semi-formal modeling techniques used in software engineering to specify, design, and verify complex software systems.

### Key Topics Covered:
- Formal specification languages and methods
- Requirements modeling and traceability
- System architecture and component modeling
- GUI and user interface modeling
- Multithreaded and concurrent system modeling
- Network and distributed system modeling
- Model verification and validation techniques

## Module 01: Modeling Basics

This foundational module introduces core concepts in software modeling and formal specification.

### Learning Objectives:
- Understand the purpose and benefits of software modeling
- Learn fundamental modeling concepts and notation
- Practice with basic specification languages
- Establish traceability between requirements and implementation

### Contents:

#### [`Module_01_Modeling_Basics/`](./Module_01_Modeling_Basics/)

**HelloWorld Example** - A simple "Hello World" program modeled using different specification approaches:

- **`HelloWorld.als`** - Alloy specification demonstrating:
  - Signature definitions for requirements and system blocks
  - Relationship modeling between components
  - Predicate logic for constraint specification
  - Model validation using the Alloy Analyzer

- **`HelloWorld.sysml`** - SysML v2 model showing:
  - Block definition and structure
  - Attribute and action specification
  - Systems engineering modeling concepts

- **`HelloWorld.java`** - Implementation that satisfies the modeled requirements:
  - Traceability from formal specification to working code
  - Documentation linking back to requirements
  - Demonstration of specification-driven development

### Key Concepts Introduced:

1. **Requirements Modeling**: Formal specification of system requirements using structured approaches
2. **Traceability**: Establishing clear links between requirements, models, and implementation
3. **Formal Specification**: Using mathematical notation and logic to precisely define system behavior
4. **Model-Driven Development**: Using models as the primary artifacts for system development
5. **Verification**: Ensuring models correctly represent intended system behavior

### Tools and Languages:
- **Alloy**: Formal specification language with built-in analysis capabilities
- **SysML v2**: Systems Modeling Language for complex system specification
- **Java**: Implementation language demonstrating model realization

This module establishes the foundation for more advanced modeling techniques covered in subsequent modules, including complex system architectures, concurrent systems, and distributed network models.
