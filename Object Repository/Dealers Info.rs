<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>All Dealers in Locate Cache</description>
   <name>Dealers Info</name>
   <tag></tag>
   <elementGuidId>a7f73597-9400-47c2-8878-11d95f262573</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003csearchrequest mode\u003d\&quot;dealer\&quot;\u003e\n    \u003csourceapplication\u003eDealerService\u003c/sourceapplication\u003e\n    \u003cbtovisitordescriptor\u003e1\u003c/btovisitordescriptor\u003e\n    \u003cconfigurationid\u003e1\u003c/configurationid\u003e\n    \u003ccriteria\u003e\n    \u003c/criteria\u003e\n    \u003cselect records\u003d\&quot;10000\&quot;\u003e\n        \u003cfields\u003e\n            \u003c!--\u003cfield name\u003d\&quot;*\&quot;/\u003e--\u003e\n            \u003cfield name\u003d\&quot;dealerPAcode\&quot;/\u003e\n            \u003cfield name\u003d\&quot;name\&quot;/\u003e\n            \u003cfield name\u003d\&quot;FordImagePreference\&quot;/\u003e\n            \u003cfield name\u003d\&quot;LincolnImagePreference\&quot;/\u003e\n            \u003cfield name\u003d\&quot;zip\&quot;/\u003e\n        \u003c/fields\u003e\n    \u003c/select\u003e\n\u003c/searchrequest\u003e&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://40.85.167.114:3005/locate-proxy?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
