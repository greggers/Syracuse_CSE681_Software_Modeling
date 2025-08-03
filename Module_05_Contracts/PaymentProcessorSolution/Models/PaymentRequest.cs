namespace PaymentProcessorSolution.Models
{
    public class PaymentRequest
    {
        public string AccountId { get; set; }
        public decimal Amount { get; set; }
        public string Currency { get; set; }
        public string PaymentMethod { get; set; }

        public bool IsValid()
        {
            return !string.IsNullOrEmpty(AccountId)
                   && Amount > 0
                   && !string.IsNullOrEmpty(Currency)
                   && !string.IsNullOrEmpty(PaymentMethod);
        }
    }
}
