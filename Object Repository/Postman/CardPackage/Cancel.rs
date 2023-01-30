<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Cancel</name>
   <tag></tag>
   <elementGuidId>49557771-e9db-4365-b992-f5c12d3274ba</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ext_transaction_id\&quot;: \&quot;${ext_transaction_id}\&quot;,\n  \&quot;tx_correlation_id\&quot;: \&quot;${tx_correlation_id}\&quot;,\n  \&quot;ext_transaction_id_ref\&quot;: \&quot;${ext_transaction_id_ref}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ca4b4b80-614b-4e52-9ca2-67e5b1f8457e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>a8017dce-6332-4bc5-b40d-f3fda4035b3d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/cp/Transaction/Cancel</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>31d77772-f2b6-4991-8246-20b56d4b7c6e</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>2971181c-0a1d-44fa-8e22-e37ff0d36a45</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>fbcbd288-9ee9-4eea-82ec-feb2d5c33811</id>
      <masked>false</masked>
      <name>ext_transaction_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5796cdf0-33ac-4e4a-97a2-eaee9325485e</id>
      <masked>false</masked>
      <name>tx_correlation_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6fc816a8-2a2d-4ce5-9254-1b70cf7f1f5a</id>
      <masked>false</masked>
      <name>ext_transaction_id_ref</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()




println(&quot;Cancel request body: \n&quot;+ request.getBodyContent().getText())

import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;Cancel response body\n&quot;+json.toString(4));

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
