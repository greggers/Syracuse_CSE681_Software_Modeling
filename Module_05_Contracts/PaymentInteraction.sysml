<?xml version="1.0" encoding="UTF-8"?>
<model name="PaymentInteraction" xmlns="http://www.omg.org/spec/SysML/20110701/SysML">
  <packagedElement name="Actors">
    <actor name="Client"/>
    <actor name="PaymentProcessor"/>
  </packagedElement>

  <packagedElement name="Interactions">
    <interaction name="ProcessPaymentSequence">
      <lifeline ref="Client"/>
      <lifeline ref="PaymentProcessor"/>
      <message name="RequestPayment" sender="Client" receiver="PaymentProcessor">
        <argument name="paymentRequest" type="PaymentRequest"/>
      </message>
      <message name="ValidatePayment" sender="PaymentProcessor" receiver="PaymentRequest">
        <argument name="paymentRequest" type="PaymentRequest"/>
      </message>
      <message name="ReturnResult" sender="PaymentProcessor" receiver="Client">
        <argument name="result" type="Boolean"/>
      </message>
    </interaction>
  </packagedElement>

  <packagedElement name="Classes">
    <class name="PaymentRequest"/>
  </packagedElement>
</model>