using System;
using InterfaceContractDemo;

namespace InterfaceContractDemo
{
    class Program
    {
        static void Main(string[] args)
        {
            ISimpleContract simpleContract = new SimpleContractImplementer();
            Console.WriteLine(simpleContract.GetGreeting());
        }
    }
}