# Module_05_Contracts

This repository contains code examples and demonstrations of **contract-based development in C#**, used in a graduate-level software engineering course at Syracuse University (CSE 681 - Software Modeling).

## 📦 Projects Included

- **InterfaceContractDemo**: A minimal example demonstrating interface-based design contracts.
- **PaymentProcessorSolution**: A more advanced example showing contracts in a payment processing system, including:
  - Service interface with method contract
  - Data validation using preconditions and postconditions

---

## 🛠️ Building the Solution

To build the solution:

1. Clone the repository:
   ```bash
   git clone https://github.com/greggers/Syracuse_CSE681_Software_Modeling.git
   cd Syracuse_CSE681_Software_Modeling/Module_05_Contracts
   ```

2. Build either project using the .NET CLI:
   ```bash
   dotnet build
   ```

Or open the solution in **Visual Studio**, **Rider**, or **VS Code** and build through the IDE.

---

## ▶️ Running the Examples

To run a specific project (e.g., `PaymentProcessorSolution`):

```bash
cd PaymentProcessorSolution
dotnet run
```

For `InterfaceContractDemo`:

```bash
cd InterfaceContractDemo
dotnet run
```

Ensure the build completes without errors before running.

---

## 🧠 Educational Focus

These examples illustrate:
- **Design by Contract (DbC)** in .NET
- Use of `interface` as a behavioral contract
- Input validation (preconditions)
- Operation success checking (postconditions)
- Practical application to service-like architecture

---

## ✅ Prerequisites

- [.NET 7 SDK](https://dotnet.microsoft.com/en-us/download/dotnet/7.0) or newer
- A modern C# development environment (VS Code, Visual Studio 2022+, Rider)

---

## 📚 License and Usage

These code examples are free to use for academic and instructional purposes.
