/**
 * HelloWorld Java Program
 * 
 * This program satisfies the requirement:
 * "Program shall output to command line 'Hello World'"
 * 
 * Based on the Alloy model specification in HelloWorld.als
 */
public class HelloWorld {
    
    private String message;
    
    /**
     * Constructor for HelloWorld block
     */
    public HelloWorld() {
        this.message = "Hello World";
    }
    
    /**
     * Display the message to command line
     */
    public void displayMessage() {
        System.out.println(message);
    }
    
    /**
     * Main method - entry point of the program
     */
    public static void main(String[] args) {
        // Create HelloWorld block instance
        HelloWorld app = new HelloWorld();
        
        // Output message to command line (satisfies requirement)
        app.displayMessage();
    }
}
