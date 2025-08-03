using System;
using PaymentProcessorSolution.Models;
using PaymentProcessorSolution.Services;

namespace PaymentProcessorSolution
{
    class Program
    {
        static void Main(string[] args)
        {
            var processor = new PaymentProcessor();

            var request = new PaymentRequest
            {
                AccountId = "user-123",
                Amount = 500.00m,
                Currency = "USD",
                PaymentMethod = "CreditCard"
            };

            try
            {
                bool result = processor.ProcessPayment(request);
                Console.WriteLine("Result: " + (result ? "Success" : "Failure"));
            }
            catch (Exception ex)
            {
                Console.WriteLine("Payment failed with error: " + ex.Message);
            }
        }
    }
}