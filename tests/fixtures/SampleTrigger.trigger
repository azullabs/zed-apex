trigger AccountTrigger on Account (before insert, before update, after insert) {
    if (Trigger.isBefore) {
        for (Account acc : Trigger.new) {
            if (acc.Name == null) {
                acc.Name = 'Default';
            }
        }
    }

    if (Trigger.isAfter && Trigger.isInsert) {
        List<Task> tasks = new List<Task>();
        for (Account acc : Trigger.new) {
            tasks.add(new Task(
                Subject = 'Follow up',
                WhatId = acc.Id
            ));
        }
        insert tasks;
    }
}
