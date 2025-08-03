using System;
using InterfaceContractDemo;

namespace InterfaceContractDemo
{
    public class SimpleContractImplementer : ISimpleContract
    {
        public string GetGreeting()
        {
            return "Hello, World!";
        }
    }
}