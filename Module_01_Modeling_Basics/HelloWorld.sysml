model HelloWorldModel {

  package requirements {

    requirement Req1 {
      id = "REQ-001";
      text = "Program shall output to command line 'Hello World'";
    }

  }

  package system {

    block HelloApp {
      properties:
        message: String = "Hello World";
      operations:
        run(): Void;
    }

  }

  package relationships {

    satisfy HelloApp_satisfies_Req1 {
      client: system::HelloApp;
      supplier: requirements::Req1;
    }

  }

}
