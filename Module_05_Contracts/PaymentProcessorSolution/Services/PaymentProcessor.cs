using System;
using PaymentProcessorSolution.Contracts;
using PaymentProcessorSolution.Models;

namespace PaymentProcessorSolution.Services
{
    public class PaymentProcessor : IPaymentProcessor
    {
        public bool ProcessPayment(PaymentRequest request)
        {
            // PRECONDITION
            if (!request.IsValid())
                throw new ArgumentException("Invalid payment request.");

            Console.WriteLine($"Processing payment for account {request.AccountId}...");

            // Simulate processing
            bool success = request.Amount < 1000; // Reject anything >= $1000 for this demo

            // POSTCONDITION
            if (success)
                Console.WriteLine("Payment processed successfully.");
            else
                Console.WriteLine("Payment failed: Amount exceeds processing limit.");

            return success;
        }
    }
}
