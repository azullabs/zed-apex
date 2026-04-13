/**
 * A sample Apex class for testing syntax highlighting and parsing.
 */
public with sharing class SampleClass extends BaseClass implements Comparable {
    private static final String CONSTANT_VALUE = 'hello';
    private String name;
    private Integer count = 0;

    public SampleClass(String name) {
        this.name = name;
    }

    public String getName() {
        return this.name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public static List<Account> getAccounts() {
        List<Account> accounts = [SELECT Id, Name FROM Account WHERE IsActive = true LIMIT 10];
        return accounts;
    }

    public void incrementCount() {
        count++;
    }

    @AuraEnabled(cacheable=true)
    public static Account findAccount(String searchName) {
        if (String.isBlank(searchName)) {
            throw new IllegalArgumentException('Name cannot be blank');
        }

        Account result = null;
        for (Account acc : [SELECT Id, Name FROM Account WHERE Name LIKE :searchName]) {
            result = acc;
            break;
        }
        return result;
    }

    public Integer compareTo(Object other) {
        SampleClass that = (SampleClass) other;
        return this.name.compareTo(that.name);
    }

    public enum Status {
        ACTIVE,
        INACTIVE,
        PENDING
    }
}
