# Regenerative Errors

## Common Error Handling Approaches

### Exceptions

Exceptions are performed primarily through try catch blocks, and through the use of the throw key work, this is the case for javascript, java, C++, C#, and others. While it has its advantages, principally it handles errors, and is fairly free of boiler plate code. However it is fairly difficult to recover from throwing an exception. 

### Integer return codes

This form of error handling is used by languages such as C, as well as operating systems, but is often opt in, which means people can easily ignore errors as they occure.

### Don't handle errors at all
Things just break unexpectedly, and nothing works properly, this tends to yield vulnerabilities for erroneous states that haven't been caught in testing phase, if one existed at all.
