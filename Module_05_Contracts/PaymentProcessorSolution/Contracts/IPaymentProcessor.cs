using PaymentProcessorSolution.Models;

namespace PaymentProcessorSolution.Contracts
{
    public interface IPaymentProcessor
    {
        bool ProcessPayment(PaymentRequest request);
    }
}
